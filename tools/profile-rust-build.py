#!/usr/bin/env python3
"""Profile a cold CI build and collect compiler phases and host memory samples."""

import json
import os
from pathlib import Path
import platform
import re
import resource
import shutil
import subprocess
import sys
from threading import Event, Thread
import time


def command_output(*command):
    try:
        return subprocess.run(
            command, check=False, capture_output=True, text=True, timeout=5
        ).stdout.strip()
    except (OSError, subprocess.TimeoutExpired):
        return ""


def write_json(path, data):
    path.write_text(json.dumps(data, indent=2) + "\n")


def process_stats():
    stats = {}
    for line in command_output("ps", "-axo", "rss=,comm=").splitlines():
        rss, _, command = line.strip().partition(" ")
        name = Path(command.strip()).name
        if name in {"rustc", "cargo", "ld", "rust-lld", "dsymutil"} and rss.isdigit():
            stats[name] = stats.get(name, 0) + int(rss) * 1024
    return stats


def collect_memory(output, stop, begin):
    with (output / "memory.jsonl").open("w") as stream:
        while True:
            sample = {"seconds": time.monotonic() - begin, "process_rss": process_stats()}
            if sys.platform == "darwin":
                sample["vm_stat"] = command_output("vm_stat")
                sample["swap"] = command_output("sysctl", "-n", "vm.swapusage")
            else:
                for name in ("meminfo", "vmstat"):
                    sample[name] = Path(f"/proc/{name}").read_text()
                sample["pressure"] = Path("/proc/pressure/memory").read_text()
            stream.write(json.dumps(sample) + "\n")
            stream.flush()
            if stop.is_set():
                break
            stop.wait(12)


def profile_compiler(output, compiler, args):
    if "--crate-name" not in args:
        os.execvp(compiler, [compiler, *args])

    crate = args[args.index("--crate-name") + 1]
    if crate == "aux_rs":
        unit = "aux_rs-unit-test" if "--test" in args else "aux_rs-library"
    else:
        unit = crate
    jobserver = re.search(
        r"--jobserver-auth=(\d+),(\d+)", os.environ.get("CARGO_MAKEFLAGS", "")
    )
    start = time.time()
    result = subprocess.run(
        [compiler, *args, "-Ztime-passes"],
        env={**os.environ, "RUSTC_BOOTSTRAP": "1"},
        stderr=subprocess.PIPE,
        pass_fds=tuple(map(int, jobserver.groups())) if jobserver else (),
    )
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    (output / f"{unit}.log").write_bytes(result.stderr)
    write_json(output / f"{unit}.json", {
        "start": start,
        "wall_seconds": time.time() - start,
        "user_seconds": usage.ru_utime,
        "system_seconds": usage.ru_stime,
        "peak_rss_bytes": usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024),
        "major_faults": usage.ru_majflt,
        "minor_faults": usage.ru_minflt,
        "input_blocks": usage.ru_inblock,
        "output_blocks": usage.ru_oublock,
        "incremental": any("incremental=" in arg for arg in args),
        "status": result.returncode,
    })
    sys.stderr.buffer.write(b"".join(
        line for line in result.stderr.splitlines(keepends=True) if not line.startswith(b"time:")
    ))
    return result.returncode


def profile_build(output):
    output.mkdir(parents=True, exist_ok=True)
    target = Path(os.environ["RUNNER_TEMP"]) / "profile-target"
    rust_suite = ["cargo", "test", "-p", "aux-rs", "--all-features", "--locked", "--no-run", "--timings"]
    cpu = (
        command_output("sysctl", "-n", "machdep.cpu.brand_string")
        if sys.platform == "darwin" else command_output("lscpu")
    )
    write_json(output / "host.json", {
        "platform": platform.platform(),
        "logical_cpus": os.cpu_count(),
        "cpu": cpu,
        "disk_free_bytes": shutil.disk_usage(output).free,
        "rustc": command_output("rustc", "--version", "--verbose"),
        "memory": (
            command_output("sysctl", "-n", "hw.memsize")
            if sys.platform == "darwin" else Path("/proc/meminfo").read_text().splitlines()[0]
        ),
    })
    mode = os.environ["AUX_BUILD_PROFILE"]
    environment = {
        **os.environ,
        "RUSTC_WORKSPACE_WRAPPER": str(Path(__file__).resolve()),
        "CARGO_TARGET_DIR": str(target),
        "CARGO_TERM_COLOR": "never",
    }
    if mode == "no-incremental":
        environment["CARGO_INCREMENTAL"] = "0"
    if mode == "integration-only":
        rust_suite.extend(["--test", "*"])
    start = time.monotonic()
    stop = Event()
    monitor = Thread(target=collect_memory, args=(output, stop, start), daemon=True)
    monitor.start()
    try:
        result = subprocess.run(rust_suite, env=environment)
        elapsed = time.monotonic() - start
    finally:
        stop.set()
        monitor.join()
    reports = target / "cargo-timings"
    if reports.is_dir():
        for report in reports.glob("cargo-timing-*.html"):
            shutil.copy2(report, output / "cargo-timing.html")
    write_json(output / "build.json", {
        "profile": mode,
        "wall_seconds": elapsed,
        "status": result.returncode,
    })
    print(f"{mode}: {elapsed:.1f}s")
    for unit in ("aux_rs-library", "aux_rs-unit-test"):
        path = output / f"{unit}.json"
        if path.is_file():
            stats = json.loads(path.read_text())
            print(f"{unit}: {stats['wall_seconds']:.1f}s, RSS {stats['peak_rss_bytes'] / 1024**3:.2f} GiB")
    return result.returncode


if __name__ == "__main__":
    output_dir = Path(os.environ["AUX_BUILD_PROFILE_OUTPUT"])
    if len(sys.argv) == 1:
        sys.exit(profile_build(output_dir))
    sys.exit(profile_compiler(output_dir, sys.argv[1], sys.argv[2:]))

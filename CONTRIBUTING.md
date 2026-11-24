# Contributing to aux

Build from the repository root with Zig 0.17.1. Shared transport and wire handling live in `src/core/`. Place exchange and data protocols under their respective vendor in `src/exchanges/` or `src/data/`. Keep the SDK wrappers in `sdk/` and extend the wire fixtures in `tests/` alongside protocol changes.

Run `zig build test` for the native core. After changing protocol sources, run `python3 rebuild.py` to refresh the archives used by the SDKs. Run `python3 tools/generate_clients.py` after editing the vendor schemas. Commit generated SDK sources and updated archives with their inputs.

Use `python3 check_zig_sdk.py` to verify the packaged Zig module and `python3 check_packages.py` for the Rust crate, Python wheel and npm tarball. These commands exercise locally installed packages against fixtures. The [build guide](README.md#build-and-verify) lists the toolchain requirements.

## Releases

Keep the Cargo workspace version and its `aux-rs` dependency requirement in sync with the Python project version and the Node manifest and lockfile. Verify the Rust archive with `cargo package -p aux-rs --locked`. The Node release workflow packs all three target add-ons into one `aux-ts` tarball. The Python release workflow builds the `aux-py` wheels and source distribution and checks an `aux-py-v<version>` release tag before publishing.

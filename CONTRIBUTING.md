# Contributing to aux

Build from the repository root with Zig 0.17.1. Shared transport and wire handling live in `src/core/`. Place exchange and data protocols under their respective vendor in `src/exchanges/` or `src/data/`. Keep the SDK wrappers in `sdk/` and extend the wire fixtures in `tests/` alongside protocol changes.

Run `zig build test` for the native core. From a source checkout, run `python3 rebuild.py` before building the Rust, Python, or Node SDKs, and rerun it after changing native protocol sources. Run `python3 tools/generate_clients.py --zig-generated-only` before using a source checkout as a Zig dependency. Run `python3 tools/generate_clients.py --typescript-only` before building the npm package from source.

After changing vendor schemas, run `python3 tools/generate_clients.py` and commit the updates to tracked SDK sources with their inputs. CI generates the native archives, Zig client modules, and TypeScript bindings for the packages that use them.

Use `python3 check_zig_sdk.py` to verify the packaged Zig module and `python3 check_packages.py` for the Rust crate, Python wheel and npm tarball. These commands exercise locally installed packages against fixtures. The [build guide](README.md#build-and-verify) lists the toolchain requirements.

## Releases

Pushes to `main` and `master` run generated SDK coverage validation and the default Rust tests on Linux. The release workflows run the native and installed-package checks and produce the downloadable packages.

Each SDK has its own workflow. Run the workflow manually on the release commit to build and test the package. Download the resulting artifact from the Actions run. Publishing a GitHub release with the matching tag runs the same checks and then publishes that SDK.

| SDK | Workflow | Example release tag | Destination |
| --- | --- | --- | --- |
| Rust | `release-rust.yml` | `aux-rs-v0.0.4` | crates.io, package `aux-rs` |
| TypeScript | `release-node.yml` | `aux-connectors-v0.0.4` | npm, package `aux-connectors` |
| Python | `release-python.yml` | `aux-py-v0.0.4` | PyPI, project `aux-py` |
| Zig | `release-zig.yml` | `aux-zig-v0.0.4` | Source archive attached to the GitHub release |

Set the version in the SDK manifest before tagging. Rust and Python share the Cargo workspace version. Update the workspace dependency on `aux-rs` and the Cargo lockfile when changing it. TypeScript uses `sdk/typescript/package.json` and its npm lockfile. Zig uses `build.zig.zon`.

A package owner must configure each registry's trusted publisher for `polaris-data/aux`, the workflow filename above, and the corresponding GitHub environment (`crates-io`, `npm`, or `pypi`). npm prereleases use the `next` dist-tag. The Zig workflow uses the repository token to attach the verified archive. Given version 0.0.4, consumers can install it with the following command after publication.

```sh
zig fetch --save=aux https://github.com/polaris-data/aux/releases/download/aux-zig-v0.0.4/aux-zig-native-0.0.4.tar.gz
```

# Contributing to aux

Build from the repository root with Zig 0.17.1. Shared transport and wire handling live in `src/core/`. Place exchange and data protocols under their respective vendor in `src/exchanges/` or `src/data/`. Keep the SDK wrappers in `sdk/` and extend the wire fixtures in `tests/` alongside protocol changes.

Run `zig build test` for the native core. After changing protocol sources, run `python3 rebuild.py` to refresh the archives used by the SDKs. Run `python3 tools/generate_clients.py` after editing the vendor schemas. Commit generated SDK sources and updated archives with their inputs.

Use `python3 check_zig_sdk.py` to verify the packaged Zig module and `python3 check_packages.py` for the Rust crate, Python wheel and npm tarball. These commands exercise locally installed packages against fixtures. The [build guide](README.md#build-and-verify) lists the toolchain requirements.

## Releases

Each SDK has its own workflow. Run the workflow manually on the release commit to build and test the package. Download the resulting artifact from the Actions run. Publishing a GitHub release with the matching tag runs the same checks and then publishes that SDK.

| SDK | Workflow | Example release tag | Destination |
| --- | --- | --- | --- |
| Rust | `release-rust.yml` | `aux-rs-v0.0.1` | crates.io, package `aux-rs` |
| TypeScript | `release-node.yml` | `aux-ts-v0.0.1` | npm, package `aux-ts` |
| Python | `release-python.yml` | `aux-py-v0.0.1` | PyPI, project `aux-py` |
| Zig | `release-zig.yml` | `aux-zig-v0.0.1` | Source archive attached to the GitHub release |

Set the version in the SDK manifest before tagging. Rust and Python share the Cargo workspace version. Update the workspace dependency on `aux-rs` and the Cargo lockfile when changing it. TypeScript uses `sdk/typescript/package.json` and its npm lockfile. Zig uses `build.zig.zon`.

A package owner must configure each registry's trusted publisher for `polaris-data/aux`, the workflow filename above, and the corresponding GitHub environment (`crates-io`, `npm`, or `pypi`). Check publishing access to the existing public npm package `aux-ts` before scheduling the release. npm prereleases use the `next` dist-tag. The Zig workflow uses the repository token to attach the verified archive. Given version 0.0.1, consumers can install it with the following command after publication.

```sh
zig fetch --save=aux https://github.com/polaris-data/aux/releases/download/aux-zig-v0.0.1/aux-zig-native-0.0.1.tar.gz
```

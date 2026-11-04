# Pinned Kalshi coverage

`coverage.json` maps named Rust and Python REST interfaces to fixtures. `schema/services.json` separates native products/services; `schema/stream-sources.json` and `schema/websocket.json` inventory upstream message and channel surfaces. `tests/contracts.rs` invokes every named REST method through injected I/O and checks method, path and exact response round-trip. `tests/websocket_contracts.rs` decodes every inventoried event fixture. Authentication, lifecycle and chain cases are handwritten tests.

Current inventory: 215 REST operations; 75 stream messages. Fixtures use upstream examples when available and explicitly synthetic schema samples otherwise. They are compatibility evidence, not proof of successful account access, matching or settlement. Batch results remain native, including mixed successes/errors.

Run `python3 scripts/check-prediction-coverage.py` offline. It checks pinned artifact hashes, method/binding parity, fixture sets and named tests. Pass `--reference-root PATH` to also compare raw/normalized inputs and regenerate declarations read-only. External inputs are deliberately not runtime or CI dependencies. REST normalizations preserve exact numbers, YAML 1.2 booleans, per-service method prefixes, missing documented path parameters and documented supplements. Polymarket supplements are tracked in `schema/supplemental.json`; stream schema/example discrepancies are recorded in `schema/stream-notes-*.json`. RTDS has an independent native schema derived from the pinned SDK.

Generate with `generate-prediction-apis.py`, then `generate-prediction-streams.py`, then `generate-prediction-python.py`; all take `--check`, and the first two require `--reference-root`. `generate-polymarket-contracts.py` emits the named calldata builders from pinned signatures. External schema inputs must match API_SOURCES.json; update pins and review operation diffs deliberately when upstream changes.

`schema/blockers.json` records publication limits and excluded scope. Missing schema fields remain exact native mappings; the connector does not fabricate undocumented settlement, deployment or private-account behavior.

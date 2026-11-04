# Pascal signing compatibility

Authentication and serialization are handwritten, with no runtime official SDK. `tests/signing.rs` compares independently produced reference bytes and tests domain/authority/input separation. Fixtures contain public test keys only. Remote signer callbacks receive final pre-sign bytes or EIP-712 payloads and their returned signatures are checked against the claimed identity. Callback failures expose structured signing errors without private material.

Source provenance is recorded in API_SOURCES.json and NOTICE. Pascal permit vectors are the official published vectors; Solana transaction bytes were generated with solders 0.27.1. Polymarket EIP-712 vectors use eth-account 0.13.7 and MessagePack uses msgpack 1.1.2; contract calldata and session envelopes use eth-abi. Kalshi Ed25519 and RSA-PSS signatures use Python cryptography with PSS salt length 32; probabilistic signatures are verified rather than compared byte-for-byte. These independent encoders are development references, never runtime or offline CI requirements.

Python signers accept explicit local test keys or asynchronous external signing callbacks. Callback context follows the initiating asyncio task and cancellation propagates to outstanding signing work. Never log signer inputs containing account secrets or reuse signing credentials across deployment/product domains.

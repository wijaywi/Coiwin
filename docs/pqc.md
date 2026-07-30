# PQC (Post-Quantum Cryptography) — Coiwin- PoC

This document describes a Proof-of-Concept (PoC) for integrating post-quantum cryptography algorithms into the Coiwin- project.

## Goals
- Provide a PoC for post-quantum signatures (Dilithium) and key exchange (Kyber).
- Demonstrate a hybrid approach: classic (secp256k1) + PQC (Dilithium).
- Develop a roadmap for developing PQC features in Coiwin-.

## Related Files
- `Coiwin-/pqc/dilithium_precompile.rs` — Rust PoC module for Dilithium verification.
- `.github/workflows/pqc-tests.yml` — workflow for automated tests.
- `README.md` — main project description.

## ⚙️ Example configuration (draft)
```yaml
pqc:
enabled: false
pqc_default_alg: "dilithium2"
pqc_kem_for_p2p: "kyber512"
allow_hybrid_tx: true

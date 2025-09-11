
211 / 5,000
#CoiwinCoin — PQC PoC
Proof-of-Concept project for the integration of Post-Quantum Cryptography (PQC), specifically Dilithium (digital signature) and Kyber (key encapsulation), into the CoiwinCoin ecosystem.

## ✨ What's this?
CoiwinCoin wants to build a blockchain that is resistant to quantum computer attacks.
The first step: adding a Dilithium signature verification module as a Proof-of-Concept (PoC).
Copyright (c) 2025 Winjay (Wiwin Wijaya).

## 📂 Repo Contents
- `Coiwind/pqc/dilithium_precompile.rs` — Rust precompile code for Dilithium verification.
- `docs/pqc.md` — PQC concept documentation & roadmap.
- `.github/workflows/pqc-tests.yml` — GitHub Actions workflow for automated testing.
- `LICENSE` — MIT license.

## 🚀 Build method (example for developers)
```bash
# build with PQC features
cargo build --workspace --features pqc-dilithium

# run test
cargo test --workspace --features pqc-dilithium
🛣️ Roadmap (initial phase)
• Add Dilithium precompile PoC
• Add Kyber KEM for P2P
• CLI wallet with hybrid tx (ECDSA + Dilithium)
• Benchmark on testnet
📜 License
MIT — free to use, modify, and distribute.

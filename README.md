# Coiwin: PQC Framework for Blockchain

The **Coiwin PQC Framework** provides an open-source Rust module for integrating **Post-Quantum Cryptography (PQC)** into blockchain architectures. Specifically, it implements the NIST-standardized **Dilithium** digital signature scheme (CRYSTALS-Dilithium) to ensure transaction security against future quantum computer attacks.

Originally developed as part of experimental research into quantum-resistant ledger technology, **Coiwin** has now been decoupled into a standalone cryptographic library to serve as an educational and technical resource for cryptography researchers and blockchain developers.

## Features

- **Dilithium Signatures:** Native Rust precompiles for Dilithium keypair generation, detached signing, and rigorous signature verification.
- **Blockchain Agnostic:** Designed to be easily integrated into any consensus mechanism or smart contract execution environment.
- **High Performance:** Built entirely in memory-safe and fast Rust.
- **Educational Foundation:** Clean, un-obfuscated code serving as a blueprint for upgrading legacy ECDSA architectures to Quantum-Resistant architectures.

## Usage & Demonstration

This repository contains a simple binary to demonstrate the core functionality of the Dilithium signature lifecycle.

To run the demonstration:
```bash
cargo run --features pqc-dilithium
```

### Expected Output
```text
=== Coiwin Quantum-Resistant Cryptography Library ===
This binary serves as a simple demonstration of Dilithium signatures.

[1] Generating Dilithium Keypair...
Public Key generated (2592 bytes)
Secret Key generated (4864 bytes)

[2] Signing message: Hello, Quantum World!
Signature created (3293 bytes)

[3] Verifying signature...
Result: VERIFICATION SUCCESSFUL! The signature is valid.
```

## Repository Structure

- `src/pqc/dilithium_precompile.rs` — Core Rust implementation of the Dilithium digital signature module.
- `src/bin/node.rs` — Demonstration CLI.
- `docs/pqc.md` — Additional documentation regarding Post-Quantum Cryptography concepts.
- `contracts/DilithiumVerifier.sol` — Solidity smart contract for Dilithium verification on EVM-compatible chains.

---

## Legal & Disclaimer

**IMPORTANT NOTICE:**
This software is provided purely as an experimental, open-source research module.

1. **Not a Cryptocurrency:** This repository DOES NOT contain any cryptocurrency, wallet, node, or mining software. It is strictly a cryptographic library.
2. **Zero Liability:** As stated in the `LICENSE` (MIT License), this software is provided **"AS IS"**. The creators, authors, and contributors accept **ZERO LIABILITY** for any financial loss, hardware damage, or legal repercussions resulting from compiling, modifying, or integrating this code. 
3. **Unaudited Code:** This module is an educational proof-of-concept and has not undergone formal security audits. Do not use in production environments holding real financial value without proper auditing.

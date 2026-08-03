#CoiwinCoin — PQC PoC

A Proof-of-Concept project for integrating Post-Quantum Cryptography (PQC), specifically Dilithium (digital signature) and Kyber (key encapsulation), into the CoiwinCoin ecosystem.

## What's this?
CoiwinCoin aims to develop a blockchain that is resistant to quantum computer attacks.

First step: Adding a Dilithium signature verification module as a Proof-of-Concept (PoC).

## Repo Contents
- `Coiwind/pqc/dilithium_precompile.rs` — Rust precompiled code for Dilithium verification.
- `docs/pqc.md` — PQC concept documentation & roadmap.
- `.github/workflows/pqc-tests.yml` — GitHub Actions workflow for automated testing.
- `LICENSE` — MIT license.

## Build method (example for developers)
```bash
# build with PQC features
cargo build --workspace --features pqc-dilithium

# run test
cargo test --workspace --features pqc-dilithium
* Roadmap (initial phase)
• Add Dilithium precompile PoC
• Add Kyber KEM for P2P
• CLI wallet with hybrid tx (ECDSA + Dilithium)
• Benchmark on testnet
* MIT license — free to use, modify, and distribute.

##->
##  How to Run Tests

This project includes unit tests for the PQC (Dilithium) module.
To run the tests locally:

```bash
# build with PQC features
cargo build --workspace --features pqc-dilithium

# run tests
cargo test --workspace --features pqc-dilithium
##->
##  How to Run Solidity Tests (Hardhat)

To test the `contracts/DilithiumVerifier.sol` contract:

1. Install Hardhat dependencies (one-time only):
```bash
npm install --save-dev hardhat @nomiclabs/hardhat-ethers ethers typescript ts-node
2. Run the deploy + test script:
npx hardhat run scripts/testDilithium.ts

#Wiwin Wijaya, 2025

---

## ⚖️ Legal & OPSEC Disclaimer

**IMPORTANT NOTICE:** 
Coiwin is an **experimental, open-source Proof-of-Concept (PoC)** software project exploring Post-Quantum Cryptography (PQC) within a blockchain architecture. 

1. **Not a Financial Product:** Coiwin is strictly a technological experiment. It is **NOT** a security, an investment product, or a financial instrument. 
2. **No Value Guarantee:** Coiwin coins possess no inherent monetary value. The creators make no promises regarding future price, market capitalization, or exchange listings.
3. **No ICO or Premine:** There is no Initial Coin Offering (ICO), no presale, and no venture capital backing. Coins are generated purely through Proof-of-Work (PoW) consensus by network participants.
4. **Zero Liability (MIT License):** As stated in the `LICENSE` file, this software is provided **"AS IS"**. The creators, authors, and contributors of Coiwin accept **ZERO LIABILITY** for any financial loss, hardware damage, or legal repercussions resulting from compiling, running, or interacting with this software. Participate at your own risk.

*By downloading, running, or interacting with the Coiwin network, you acknowledge and agree to these terms.*

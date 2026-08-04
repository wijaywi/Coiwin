# Coiwin — Quantum-Resistant L1 Blockchain

An open-source Layer-1 Blockchain Mainnet integrating Post-Quantum Cryptography (PQC), specifically Dilithium (digital signatures) and Kyber (key encapsulation), secured by a dynamic Proof-of-Work (PoW) consensus.

## What's this?
CoiwinCoin aims to develop a blockchain that is resistant to quantum computer attacks.

First step: A fully functional Proof-of-Work (PoW) Mainnet with Dilithium signature verification to secure hybrid transactions.

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
• Launch PoW Mainnet with Dilithium hybrid wallets
• Add Kyber KEM for P2P
• CLI wallet with hybrid tx (ECDSA + Dilithium)
• Benchmark on testnet
* MIT license — free to use, modify, and distribute.

## CLI Node Usage Guide __________________________________________________________________________________________
When you run `node.exe`, you will enter the Coiwin interactive terminal. The following commands are available:

- `mine`: Start mining a new block. If successful, you receive a 50 Coiwin block reward.
- `balance`: Display your current wallet balance and the number of pending transactions in your local mempool.
- `accounts`: Display the Coiwin Rich List (all network addresses and their respective balances).
- `status`: Show current blockchain height, latest block hash, current mining difficulty, and number of connected P2P peers.
- `connect <ip:port>`: Connect to another Coiwin node (e.g., `connect 12.34.56.78:8000`).
- `send <address> <amount>`: Send Coiwin to another user's public address (e.g., `send 024c397f... 15`).
- `exit`: Safely shut down the node.
## ________________________________________________________________________________________________________________
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


---

# ⚖️ Legal & OPSEC Disclaimer

**IMPORTANT NOTICE:** 
Coiwin is an **experimental, open-source Layer-1 Mainnet** project exploring Post-Quantum Cryptography (PQC) within a Proof-of-Work (PoW) blockchain architecture. 

1. **Not a Financial Product:** Coiwin is strictly a technological experiment. It is **NOT** a security, an investment product, or a financial instrument. 
2. **No Value Guarantee:** Coiwin coins possess no inherent monetary value. The creators make no promises regarding future price, market capitalization, or exchange listings.
3. **No ICO or Premine:** There is no Initial Coin Offering (ICO), no presale, and no venture capital backing. Coins are generated purely through Proof-of-Work (PoW) consensus by network participants.
4. **Zero Liability (MIT License):** As stated in the `LICENSE` file, this software is provided **"AS IS"**. The creators, authors, and contributors of Coiwin accept **ZERO LIABILITY** for any financial loss, hardware damage, or legal repercussions resulting from compiling, running, or interacting with this software. Participate at your own risk.

*By downloading, running, or interacting with the Coiwin network, you acknowledge and agree to these terms.*

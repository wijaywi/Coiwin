# Coiwin: A Quantum-Resistant Peer-to-Peer Electronic Cash System
**Wiwin Wijaya**  
*August 2026*

---

## Abstract
A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. While existing networks like Bitcoin provide a robust framework, the imminent advent of large-scale quantum computers threatens the foundational cryptographic primitives (specifically ECDSA) upon which these networks rely. We propose **Coiwin**, a next-generation blockchain architecture that integrates Post-Quantum Cryptography (PQC). By implementing a **Hybrid Transaction Model**—requiring both classical ECDSA and quantum-resistant Dilithium signatures—Coiwin ensures an immediate transition to quantum safety without sacrificing backwards compatibility. The network is secured by a dynamic Proof-of-Work (PoW) consensus mechanism and operates over a fully decentralized TCP-based peer-to-peer network.

---

## 1. Introduction
The transition to quantum computing poses an existential threat to modern cryptography. Shor's algorithm, running on a sufficiently powerful quantum computer, can easily break the Elliptic Curve Digital Signature Algorithm (ECDSA) used by Bitcoin and Ethereum. If compromised, attackers could forge signatures and steal funds from any address. 

Coiwin solves this by introducing an out-of-the-box, quantum-resistant cryptocurrency ecosystem. Instead of waiting for a catastrophic quantum event to force a hard fork, Coiwin proactively secures the ledger using **Dilithium**, a NIST-standardized Post-Quantum cryptographic signature scheme, operating in tandem with classical ECDSA.

## 2. The Hybrid Transaction Model
To maintain robustness against both classical and quantum attack vectors, Coiwin employs a dual-signature architecture. 

When a user initiates a transfer, the payload (`sender`, `receiver`, `amount`, `nonce`) must be signed by two distinct private keys:
1. **Classical Signature:** ECDSA (secp256k1)
2. **Quantum-Resistant Signature:** CRYSTALS-Dilithium

```rust
pub struct HybridTransaction {
    pub payload: TransactionPayload,
    pub ecdsa_signature: String,     
    pub dilithium_signature: String, 
    pub dilithium_public: String,    
}
```
A transaction is only accepted into the Mempool if **both** signatures mathematically resolve to their respective public keys. Even if a quantum adversary breaks the ECDSA signature, the Dilithium signature remains computationally infeasible to forge, rendering the funds strictly unspendable by malicious actors.

## 3. Proof-of-Work and Consensus
To achieve distributed consensus without a central authority, Coiwin utilizes a Proof-of-Work (PoW) system based on SHA-256 hashing. 

Miners bundle unconfirmed transactions from their local Mempool into a new block. The block header includes the hash of the previous block, a timestamp, a merkle root of transactions, and a cryptographic `nonce`. The miner repeatedly increments the `nonce` until the SHA-256 hash of the block header yields a value with a specific number of leading zeroes, defined by the network's `difficulty`.

**Dynamic Difficulty Adjustment:**  
The network aims for a steady block generation time. Coiwin recalculates the mining difficulty every 5 blocks. If the previous 5 blocks were mined too quickly, the difficulty increases. If they were mined too slowly, the difficulty decreases. This self-regulating mechanism ensures the integrity and predictability of coin emission regardless of the total network hashrate.

## 4. Decentralized Peer-to-Peer Network
Coiwin operates on a flat, decentralized P2P topology running over standard TCP sockets (default port: 8000). Nodes are entirely independent and maintain their own copy of the blockchain ledger.

The protocol relies on four core messages:
1. `NewTransaction(tx)`: Broadcasts unconfirmed transactions to all peers to populate Mempools.
2. `NewBlock(block)`: Broadcasts newly mined blocks. Peers immediately halt current mining operations if the received block is valid and extends the chain.
3. `RequestBlocks`: Sent by new nodes joining the network to request the ledger history.
4. `SendBlocks(chain)`: Nodes reply with their longest chain to synchronize new peers.

By prioritizing the "Longest Chain Rule", nodes inherently agree on the single source of truth. If divergent chains are detected, nodes will drop their shorter local chain and adopt the longer one, recalculating account balances recursively from the genesis block.

## 5. Tokenomics and Incentive Structure
By convention, the first transaction in a block is a special transaction (the **Coinbase Transaction**) that generates new coins owned by the creator of the block. This adds an incentive for nodes to support the network and provides a way to initially distribute coins into circulation.

- **Block Reward:** Fixed at 50 Coiwin per block.
- **Pre-mine:** Zero. Coiwin is launched fairly; all coins in existence are generated purely through computational effort (mining).
- **Transaction Fees:** Currently zero by default, allowing free peer-to-peer transfers. As the network matures, fees may be introduced to prioritize Mempool inclusion.

## 6. Conclusion
We have proposed a robust, decentralized, and quantum-resistant system for electronic transactions without relying on trust. Coiwin's architecture proves that next-generation Post-Quantum Cryptography can be seamlessly integrated into a traditional blockchain model today, ensuring the safety of digital assets long into the quantum future.

---

### Legal Disclaimer
*Coiwin is an experimental, open-source Proof-of-Concept (PoC) software project. It is strictly a technological experiment and NOT a financial product, security, or investment instrument. The creators accept ZERO LIABILITY for any financial loss, hardware damage, or legal repercussions resulting from compiling, running, or interacting with this software. Participate at your own risk.*

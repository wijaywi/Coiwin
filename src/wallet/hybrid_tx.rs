use anyhow::{anyhow, Result};
use k256::ecdsa::{signature::Signer, signature::Verifier, Signature as EcdsaSignature, SigningKey, VerifyingKey};
use pqcrypto_dilithium::dilithium5::{PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey};
use pqcrypto_traits::sign::{PublicKey, SecretKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use crate::pqc::dilithium_precompile;
use hex;

#[derive(Serialize, Deserialize)]
pub struct HybridWallet {
    pub ecdsa_secret: String,     // hex encoded bytes
    pub dilithium_secret: String, // hex encoded bytes
    pub ecdsa_public: String,
    pub dilithium_public: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionPayload {
    pub sender: String, // ecdsa_public hex
    pub receiver: String,
    pub amount: u64,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HybridTransaction {
    pub payload: TransactionPayload,
    pub ecdsa_signature: String,     // hex encoded
    pub dilithium_signature: String, // hex encoded
    pub dilithium_public: String,    // hex encoded pubkey for verification
}

impl HybridWallet {
    pub fn generate() -> Result<Self> {
        let ecdsa_sk = SigningKey::random(&mut OsRng);
        let ecdsa_pk = VerifyingKey::from(&ecdsa_sk);

        let (dilithium_pk, dilithium_sk) = dilithium_precompile::generate_keypair();

        Ok(HybridWallet {
            ecdsa_secret: hex::encode(ecdsa_sk.to_bytes()),
            ecdsa_public: hex::encode(ecdsa_pk.to_sec1_bytes()),
            dilithium_secret: hex::encode(dilithium_sk.as_bytes()),
            dilithium_public: hex::encode(dilithium_pk.as_bytes()),
        })
    }

    pub fn sign_transaction(&self, payload: &TransactionPayload) -> Result<HybridTransaction> {
        // Deserialize keys
        let ecdsa_sk_bytes = hex::decode(&self.ecdsa_secret)?;
        let ecdsa_sk = SigningKey::from_slice(ecdsa_sk_bytes.as_slice())
            .map_err(|e| anyhow!("Invalid ECDSA secret key: {}", e))?;

        let dilithium_sk_bytes = hex::decode(&self.dilithium_secret)?;
        let dilithium_sk = DilithiumSecretKey::from_bytes(&dilithium_sk_bytes)
            .map_err(|_| anyhow!("Invalid Dilithium secret key bytes"))?;

        // Serialize payload to bytes for signing
        let payload_bytes = serde_json::to_vec(payload)?;

        // ECDSA sign
        let ecdsa_sig: EcdsaSignature = ecdsa_sk.sign(&payload_bytes);

        // Dilithium sign
        let dilithium_sig_bytes = dilithium_precompile::sign_detached(&dilithium_sk, &payload_bytes)?;

        Ok(HybridTransaction {
            payload: payload.clone(),
            ecdsa_signature: hex::encode(ecdsa_sig.to_bytes()),
            dilithium_signature: hex::encode(dilithium_sig_bytes),
            dilithium_public: self.dilithium_public.clone(),
        })
    }
}

impl HybridTransaction {
    pub fn verify(&self) -> Result<bool> {
        let payload_bytes = serde_json::to_vec(&self.payload)?;

        // ECDSA Verify
        let ecdsa_pk_bytes = hex::decode(&self.payload.sender)?;
        let ecdsa_pk = VerifyingKey::from_sec1_bytes(&ecdsa_pk_bytes)
            .map_err(|e| anyhow!("Invalid ECDSA public key: {}", e))?;

        let ecdsa_sig_bytes = hex::decode(&self.ecdsa_signature)?;
        let ecdsa_sig = EcdsaSignature::try_from(ecdsa_sig_bytes.as_slice())
            .map_err(|e| anyhow!("Invalid ECDSA signature: {}", e))?;

        if ecdsa_pk.verify(&payload_bytes, &ecdsa_sig).is_err() {
            return Ok(false);
        }

        // Dilithium Verify
        let dilithium_pk_bytes = hex::decode(&self.dilithium_public)?;
        let dilithium_pk = DilithiumPublicKey::from_bytes(&dilithium_pk_bytes)
            .map_err(|_| anyhow!("Invalid Dilithium public key bytes"))?;

        let dilithium_sig_bytes = hex::decode(&self.dilithium_signature)?;
        let is_dilithium_valid = dilithium_precompile::verify_dilithium(&dilithium_pk, &payload_bytes, &dilithium_sig_bytes)?;

        Ok(is_dilithium_valid)
    }
}

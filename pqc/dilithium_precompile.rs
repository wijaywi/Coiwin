// Cowind/pqc/dilithium_precompile.rs
// PoC precompile / host function for verifying CRYSTALS-Dilithium signatures
// Feature-guarded behind Cargo feature `pqc-dilithium`.

#![allow(dead_code)]

#[cfg(feature = "pqc-dilithium")]
pub mod dilithium_precompile {
    use anyhow::Result;
    use pqcrypto_dilithium::dilithium2;
    use pqcrypto_dilithium::dilithium2::{DetachedSignature, PublicKey, SecretKey};

    /// Verify a Dilithium detached signature.
    pub fn verify_dilithium(pk_bytes: &[u8], msg: &[u8], sig_bytes: &[u8]) -> Result<bool> {
        let pk = match PublicKey::from_bytes(pk_bytes) {
            Ok(v) => v,
            Err(_) => return Ok(false),
        };
        let sig = match DetachedSignature::from_bytes(sig_bytes) {
            Ok(v) => v,
            Err(_) => return Ok(false),
        };
        Ok(dilithium2::verify_detached_signature(&sig, msg, &pk).is_ok())
    }

    /// Generate a keypair (for test/dev use only)
    pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let (pk, sk) = dilithium2::keypair();
        (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
    }

    /// Create detached signature using provided secret key bytes.
    pub fn sign_detached(sk_bytes: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
        let sk = SecretKey::from_bytes(sk_bytes).map_err(|_e| anyhow::anyhow!("invalid sk"))?;
        let sig = dilithium2::detached_sign(msg, &sk);
        Ok(sig.as_bytes().to_vec())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn dilithium_sign_verify_roundtrip() {
            let (pk, sk) = generate_keypair();
            let msg = b"test message for Cowin pqc";
            let sig = sign_detached(&sk, msg).expect("sign");
            let ok = verify_dilithium(&pk, msg, &sig).expect("verify");
            assert!(ok, "valid signature should verify");
        }

        #[test]
        fn dilithium_rejects_tampered_sig() {
            let (pk, sk) = generate_keypair();
            let msg = b"another message";
            let mut sig = sign_detached(&sk, msg).expect("sign");
            sig[0] = sig[0].wrapping_add(1);
            let ok = verify_dilithium(&pk, msg, &sig).expect("verify");
            assert!(!ok, "tampered signature must not verify");
        }
    }
}

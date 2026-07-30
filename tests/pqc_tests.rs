// tests/pqc_tests.rs
// Simple test for Dilithium PoC

#[cfg(feature = "pqc-dilithium")]
mod tests {
    use coiwin::pqc::dilithium_precompile::{generate_keypair, sign_detached, verify_dilithium};

    #[test]
    fn test_dilithium_signature_roundtrip() {
        let (pk, sk) = generate_keypair();
        let msg = b"hello coiwin PQC!";
        let sig = sign_detached(&sk, msg).expect("should sign");
        let ok = verify_dilithium(&pk, msg, &sig).expect("should verify");
        assert!(ok, "valid signature must verify");
    }

    #[test]
    fn test_dilithium_signature_tampered() {
        let (pk, sk) = generate_keypair();
        let msg = b"this message will be tampered";
        let mut sig = sign_detached(&sk, msg).expect("should sign");
        // corrupt the signature
        sig[0] ^= 0xFF;
        let ok = verify_dilithium(&pk, msg, &sig).expect("should verify");
        assert!(!ok, "tampered signature must fail");
    }
}

#[cfg(feature = "pqc-kyber")]
mod kyber_tests {
    use coiwin::pqc::kyber_kem::{generate_keypair, encapsulate_secret, decapsulate_secret};

    #[test]
    fn test_kyber_encapsulate_decapsulate() {
        let (pk, sk) = generate_keypair();
        
        // Alice encapsulates a secret for Bob using Bob's public key
        let (ct, alice_shared_secret) = encapsulate_secret(&pk);
        
        // Bob decapsulates the ciphertext using his secret key
        let bob_shared_secret = decapsulate_secret(&ct, &sk);
        
        // The shared secrets must match
        assert_eq!(
            alice_shared_secret.as_bytes(),
            bob_shared_secret.as_bytes(),
            "Shared secrets should match"
        );
    }
}

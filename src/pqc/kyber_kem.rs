use pqcrypto_kyber::kyber512::{
    decapsulate, encapsulate, keypair, Ciphertext, PublicKey, SecretKey, SharedSecret,
};

/// Generates a new Kyber KEM keypair.
pub fn generate_keypair() -> (PublicKey, SecretKey) {
    keypair()
}

/// Encapsulates a shared secret using the provided public key.
/// Returns the ciphertext and the encapsulated shared secret.
pub fn encapsulate_secret(pk: &PublicKey) -> (Ciphertext, SharedSecret) {
    let (ss, ct) = encapsulate(pk);
    (ct, ss)
}

/// Decapsulates a ciphertext using the provided secret key.
/// Returns the recovered shared secret.
pub fn decapsulate_secret(ct: &Ciphertext, sk: &SecretKey) -> SharedSecret {
    decapsulate(ct, sk)
}

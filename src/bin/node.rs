use anyhow::Result;
use coiwin::pqc::dilithium_precompile;
use pqcrypto_traits::sign::{PublicKey, SecretKey};

fn main() -> Result<()> {
    println!("=== Coiwin Quantum-Resistant Cryptography Library ===");
    println!("Notice: Full Blockchain features (Wallet, PoW, P2P) have been removed from this public repository for security reasons.");
    println!("This binary now serves as a simple demonstration of Dilithium signatures.\n");

    println!("[1] Generating Dilithium Keypair...");
    let (pk, sk) = dilithium_precompile::generate_keypair();
    println!("Public Key generated ({} bytes)", pk.as_bytes().len());
    println!("Secret Key generated ({} bytes)\n", sk.as_bytes().len());

    let message = b"Hello, Quantum World!";
    println!("[2] Signing message: {:?}", std::str::from_utf8(message).unwrap());
    let signature = dilithium_precompile::sign_detached(&sk, message)?;
    println!("Signature created ({} bytes)\n", signature.len());

    println!("[3] Verifying signature...");
    let is_valid = dilithium_precompile::verify_dilithium(&pk, message, &signature)?;
    if is_valid {
        println!("Result: VERIFICATION SUCCESSFUL! The signature is valid.");
    } else {
        println!("Result: VERIFICATION FAILED!");
    }

    Ok(())
}

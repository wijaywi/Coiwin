use std::env;
use std::process;
use pqcrypto_dilithium::dilithium5::{SecretKey, PublicKey, detached_sign, verify_detached_signature, DetachedSignature};
use pqcrypto_traits::sign::{SecretKey as TraitSecretKey, PublicKey as TraitPublicKey, DetachedSignature as TraitDetachedSignature};
use hex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: coywin_pqc <sign|verify> <key_hex> <message> [signature_hex]");
        process::exit(1);
    }

    let command = &args[1];
    let key_hex = &args[2];
    let message = args[3].as_bytes();

    if command == "sign" {
        let secret_bytes = hex::decode(key_hex).expect("Invalid hex for secret key");
        let sk = SecretKey::from_bytes(&secret_bytes).expect("Invalid secret key bytes");
        
        let sig = detached_sign(message, &sk);
        println!("{}", hex::encode(sig.as_bytes()));
    } else if command == "verify" {
        if args.len() < 5 {
            eprintln!("Missing signature_hex for verify");
            process::exit(1);
        }
        let public_bytes = hex::decode(key_hex).expect("Invalid hex for public key");
        let pk = PublicKey::from_bytes(&public_bytes).expect("Invalid public key bytes");
        
        let sig_hex = &args[4];
        let sig_bytes = hex::decode(sig_hex).expect("Invalid hex for signature");
        let signature = DetachedSignature::from_bytes(&sig_bytes).expect("Invalid signature bytes");
        
        match verify_detached_signature(&signature, message, &pk) {
            Ok(_) => {
                println!("VALID");
            }
            Err(_) => {
                println!("INVALID");
                process::exit(1);
            }
        }
    } else {
        eprintln!("Unknown command: {}", command);
        process::exit(1);
    }
}

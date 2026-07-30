use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use coiwin::wallet::hybrid_tx::{HybridTransaction, HybridWallet, TransactionPayload};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(author, version, about = "Coiwin Hybrid PQC Wallet CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a new hybrid keypair and saves it to a file
    Generate {
        #[arg(short, long, default_value = "wallet.json")]
        out: PathBuf,
    },
    /// Creates a hybrid transaction
    Transfer {
        #[arg(short, long)]
        wallet: PathBuf,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long, default_value = "tx.json")]
        out: PathBuf,
    },
    /// Verifies a hybrid transaction
    Verify {
        #[arg(short, long)]
        tx: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate { out } => {
            println!("Generating new Hybrid Wallet (ECDSA + Dilithium)...");
            let wallet = HybridWallet::generate().context("Failed to generate wallet")?;
            let json = serde_json::to_string_pretty(&wallet)?;
            fs::write(out, json).context("Failed to write wallet to file")?;
            println!("Wallet successfully saved to: {}", out.display());
            println!("ECDSA Public Address: {}", wallet.ecdsa_public);
        }
        Commands::Transfer { wallet, to, amount, out } => {
            println!("Reading wallet from {}...", wallet.display());
            let wallet_data = fs::read_to_string(wallet).context("Failed to read wallet file")?;
            let hybrid_wallet: HybridWallet = serde_json::from_str(&wallet_data).context("Invalid wallet format")?;

            let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            
            let payload = TransactionPayload {
                sender: hybrid_wallet.ecdsa_public.clone(),
                receiver: to.clone(),
                amount: *amount,
                nonce,
            };

            println!("Signing hybrid transaction...");
            let tx = hybrid_wallet.sign_transaction(&payload).context("Failed to sign transaction")?;
            
            let tx_json = serde_json::to_string_pretty(&tx)?;
            fs::write(out, tx_json).context("Failed to write transaction file")?;
            println!("Hybrid transaction successfully created and saved to: {}", out.display());
        }
        Commands::Verify { tx } => {
            println!("Reading transaction from {}...", tx.display());
            let tx_data = fs::read_to_string(tx).context("Failed to read transaction file")?;
            let hybrid_tx: HybridTransaction = serde_json::from_str(&tx_data).context("Invalid transaction format")?;

            println!("Verifying Hybrid Security (ECDSA + Dilithium signatures)...");
            let is_valid = hybrid_tx.verify().context("Error during verification process")?;
            
            if is_valid {
                println!("✅ Transaction is VALID!");
            } else {
                println!("❌ Transaction is INVALID (Signature mismatch).");
            }
        }
    }

    Ok(())
}

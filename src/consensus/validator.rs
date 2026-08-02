use crate::blockchain::chain::Blockchain;
use crate::wallet::hybrid_tx::HybridTransaction;
use anyhow::{anyhow, Result};

pub struct Validator;

impl Validator {
    pub fn validate_transaction(chain: &Blockchain, tx: &HybridTransaction) -> Result<bool> {
        // Coinbase or Genesis Tx has no sender or "COINBASE"
        if tx.payload.sender.is_empty() || tx.payload.sender == "COINBASE" {
            return Ok(true);
        }

        // Verify cryptographic hybrid signatures
        if !tx.verify()? {
            return Err(anyhow!("Invalid hybrid signatures. Transaction rejected."));
        }

        // Verify balance
        let balance = chain.get_balance(&tx.payload.sender);
        if balance < tx.payload.amount {
            return Err(anyhow!("Insufficient balance."));
        }

        Ok(true)
    }

    pub fn validate_transactions(chain: &Blockchain, txs: &[HybridTransaction]) -> Result<bool> {
        for tx in txs {
            if !Self::validate_transaction(chain, tx)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

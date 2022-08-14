use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: String,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &str, recipient: &str, amount: &str, signature: &str) -> Self {
        Self {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount: amount.to_string(),
            signature: signature.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub previous_hash: String,
    pub index: usize,
    pub transactions: Vec<Transaction>,
    pub proof: i32,
}

impl Block {
    pub fn new(
        index: usize,
        previous_hash: &str,
        transactions: Vec<Transaction>,
        proof: i32,
    ) -> Self {
        Self {
            index,
            previous_hash: previous_hash.to_string(),
            transactions,
            proof,
        }
    }
}

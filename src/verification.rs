use crate::entities::Transaction;
use crate::utils::{hash_block, hash_string_sha256};
use crate::wallet::Wallet;
use crate::Chain;

pub struct Verification;

impl Verification {
    pub fn verify_chain(chain: &Chain) -> bool {
        for (i, b) in chain.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if b.previous_hash != hash_block(chain.get(i - 1).unwrap()) {
                return false;
            }
            //remove last reward transaction
            let temp_tr = &b.transactions[0..b.transactions.len() - 1];
            if !Verification::valid_proof(temp_tr, &b.previous_hash, b.proof) {
                println!("Proof of work is invalid");
                return false;
            }
        }
        true
    }

    pub fn verify_transaction(transaction: &Transaction, balance: i32) -> bool {
        balance >= transaction.amount.parse::<i32>().unwrap()
            && Wallet::verify_transaction(transaction)
    }

    pub fn valid_proof(transactions: &[Transaction], prev_hash: &str, proof: i32) -> bool {
        let tr = serde_json::to_string(transactions).unwrap();
        let combined = format!("{}{}{}", tr, prev_hash, proof);
        let hash = hash_string_sha256(&combined);
        println!("{}", hash);
        &hash[0..3] == "000"
    }
}

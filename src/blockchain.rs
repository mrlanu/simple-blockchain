use serde::Serialize;

use crate::entities::{Block, Transaction};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::utils::hash_block;
use crate::verification::Verification;
use crate::wallet::Wallet;
use crate::{Chain, PATH, REWARD};

#[derive(Debug, Serialize, Clone)]
pub struct Blockchain {
    pub hosting_node_id: String,
    pub chain: Chain,
    pub transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(hosting_node_id: &str) -> Self {
        let _participants: HashSet<String> = HashSet::new();
        let mut chain = Vec::new();
        //Genesis block
        chain.push(Block::new(0, "", Vec::new(), 0));
        let mut transactions: Vec<Transaction> = Vec::new();
        let (stored_chain, stored_transactions) = Blockchain::load_data();
        if stored_chain.len() > 0 {
            chain = stored_chain;
        }
        if stored_transactions.len() > 0 {
            transactions = stored_transactions;
        }
        Self {
            hosting_node_id: hosting_node_id.to_string(),
            chain,
            transactions,
        }
    }

    pub fn set_hosting_node_id(&mut self, id: &str) {
        self.hosting_node_id = id.to_string();
    }

    pub fn proof_of_work(&self) -> i32 {
        let last_block = self.chain.get(self.chain.len() - 1).unwrap();
        let prev_hash = hash_block(last_block);
        let mut proof = 0;
        while !Verification::valid_proof(&self.transactions, &prev_hash, proof) {
            proof += 1;
        }
        proof
    }

    pub fn get_balance(&self) -> i32 {
        let mut balance = 0;

        self.chain.iter().for_each(|b| {
            b.transactions.iter().for_each(|t| {
                if t.sender == self.hosting_node_id {
                    balance -= &t.amount.parse().expect("Error");
                };
                if t.recipient == self.hosting_node_id {
                    balance += &t.amount.parse().expect("Error");
                };
            });
        });

        self.transactions.iter().for_each(|t| {
            if t.sender == self.hosting_node_id {
                balance -= &t.amount.parse().expect("Error");
            };
            if t.recipient == self.hosting_node_id {
                balance += &t.amount.parse().expect("Error");
            }
        });

        balance
    }

    pub fn add_transaction(&mut self, recipient: String, amount: String, signature: &str) -> bool {
        let transaction = Transaction::new(&self.hosting_node_id, &recipient, &amount, signature);
        return if Verification::verify_transaction(&transaction, self.get_balance()) {
            self.transactions.push(transaction);
            self.save_data();
            true
        } else {
            false
        };
    }

    pub fn mine_block(&mut self) -> Result<Block, String> {
        let last_block = self.chain.get(self.chain.len() - 1).expect("Error");
        let prev_hash = hash_block(&last_block);
        let mut transactions = self.transactions.clone();
        if transactions.iter().any(|t| {
            return !Wallet::verify_transaction(t);
        }) {
            println!("Error. There is bad transaction.");
            return Err("Error. There is bad transaction".to_string());
        }
        let reward_transaction =
            Transaction::new("SYSTEM", &self.hosting_node_id, &REWARD.to_string(), "");
        transactions.push(reward_transaction);
        let proof = self.proof_of_work();
        let block = Block::new(self.chain.len(), &prev_hash, transactions, proof);
        self.chain.push(block.clone());
        self.transactions.clear();
        self.save_data();
        Ok(block)
    }

    fn save_data(&self) {
        let chain_json = serde_json::to_string(&self.chain).unwrap();
        let tr_json = serde_json::to_string(&self.transactions).unwrap();
        let mut file = File::create(PATH).expect("Error");
        file.write_all(chain_json.as_bytes()).expect("Error");
        file.write_all("\n".as_bytes()).expect("Error");
        file.write_all(tr_json.as_bytes()).expect("Error");
    }

    fn load_data() -> (Chain, Vec<Transaction>) {
        let mut chain: Chain = Vec::new();
        let mut transactions: Vec<Transaction> = Vec::new();
        let input = File::open(PATH).expect("Error");
        let buffered = BufReader::new(input);
        for (i, input) in buffered.lines().enumerate() {
            if i == 0 {
                chain = serde_json::from_str(&input.unwrap()).unwrap();
            } else {
                transactions = serde_json::from_str(&input.unwrap()).unwrap();
            }
        }
        (chain, transactions)
    }
}

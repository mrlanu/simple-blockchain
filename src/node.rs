use std::fs::File;
use std::io;

use crate::blockchain::Blockchain;
use crate::verification::Verification;
use crate::wallet::Wallet;
use crate::Block;

#[derive(Debug)]
pub struct Node {
    pub wallet: Wallet,
    pub blockchain: Blockchain,
}

impl Node {
    pub fn new() -> Self {
        //let id = Uuid::new_v4().to_string();
        let mut wallet = Wallet::new();
        if File::open("keypair").is_ok() {
            wallet.load_keys().unwrap();
        } else {
            wallet.create_keys();
        }
        let blockchain = Blockchain::new(&wallet.id);
        Self { wallet, blockchain }
    }

    fn get_user_choice(&self) -> String {
        let mut choice = String::new();
        println!("Please choose:");
        println!("1 - add new transaction value");
        println!("2 - print blockchain");
        println!("3 - mine block");
        println!("4 - print participants");
        println!("5 - create wallet");
        println!("6 - load wallet");
        println!("h - hack");
        println!("q - quit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");
        choice
    }

    pub fn listen_for_input(&mut self) {
        loop {
            match self.get_user_choice().trim() {
                "1" => {
                    let (recipient, amount) = self.get_transaction_value();
                    let signature =
                        self.wallet
                            .sign_transaction(&self.wallet.id, &recipient, &amount);
                    self.blockchain
                        .add_transaction(recipient, amount, &signature);
                }
                "2" => {
                    self.print_blockchain();
                    println!(
                        "Balance of {}: {}",
                        self.blockchain.hosting_node_id,
                        self.blockchain.get_balance()
                    );
                }
                "3" => {
                    self.blockchain.mine_block().expect("TODO: panic message");
                }
                "4" => {}
                "5" => {
                    self.wallet = Wallet::new();
                    self.wallet.create_keys();
                    self.blockchain.set_hosting_node_id(&self.wallet.id);
                }
                "6" => {}
                "h" => {
                    self.blockchain.chain[0] = Block::new(0, "1", Vec::new(), 0);
                }
                "q" => break,
                _ => println!("Unknown choice"),
            }
            if !Verification::verify_chain(&self.blockchain.chain) {
                println!("Blockchain hacked !");
                break;
            }
        }
    }

    fn get_transaction_value(&self) -> (String, String) {
        let mut recipient = String::new();
        let mut amount = String::new();
        println!("Enter the recipient of the transaction:");
        io::stdin()
            .read_line(&mut recipient)
            .expect("Failed to read a recipient");
        println!("Enter the amount of the transaction:");
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read a recipient");
        //let amount: f32 = amount.trim().parse().expect("Please type a number");
        (recipient.trim().to_string(), amount.trim().to_string())
    }

    fn print_blockchain(&self) {
        println!();
        println!("<----  BLOCKCHAIN  ---->");
        self.blockchain.chain.iter().for_each(|b| {
            println!("{:?}", b);
            println!("----");
        });
        println!();
    }
}

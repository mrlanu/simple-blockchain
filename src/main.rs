mod blockchain;
mod entities;
mod node;
mod routes;
mod utils;
mod verification;
mod wallet;

pub type Chain = Vec<Block>;
use crate::entities::{Block, Transaction};
use crate::node::Node;
use actix_web::{web, App, HttpServer};
use routes::{add_transaction, get_balance, get_chain, get_open_transactions, mine_block};
use std::sync::Mutex;

const _OWNER: &str = "lanu";
const REWARD: i32 = 10;
const PATH: &str = "blockchain.txt";

pub struct AppState {
    node: Mutex<Node>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let node = web::Data::new(AppState {
        node: Mutex::new(Node::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(node.clone())
            .service(mine_block)
            .service(mine_block)
            .service(get_open_transactions)
            .service(add_transaction)
            .service(get_balance)
            .route("/", web::get().to(get_chain))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

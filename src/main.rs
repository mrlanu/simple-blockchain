// #[macro_use]
// extern crate rocket;
mod blockchain;
mod entities;
mod node;
//mod routes;
mod utils;
mod verification;
mod wallet;

// use rocket::serde::{json::Json, Deserialize};
// use rocket::tokio::sync::Mutex;
// use rocket::State;
// use serde_json::{json, Value};

pub type Chain = Vec<Block>;
use crate::entities::Block;
use crate::node::Node;
// use routes::*;

const _OWNER: &str = "lanu";
const REWARD: i32 = 10;
const PATH: &str = "blockchain.txt";

//
// type ID = usize;
//
// type ThisNode = Mutex<Node>;
// type StateNode<'r> = &'r State<ThisNode>;
//
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![get_block, get_chain, mine])
//         .register("/", catchers![not_found])
//         .manage(ThisNode::new(Node::new()))
// }

fn main() {
    Node::new().listen_for_input();
}

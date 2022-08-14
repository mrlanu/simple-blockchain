mod blockchain;
mod entities;
mod node;
mod utils;
mod verification;
mod wallet;

pub type Chain = Vec<Block>;
use crate::entities::Block;
use crate::node::Node;

const _OWNER: &str = "lanu";
const REWARD: i32 = 10;
const PATH: &str = "blockchain.txt";

fn main() {
    Node::new().listen_for_input();
}

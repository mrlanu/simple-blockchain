use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use crate::{Block, StateNode};

type ID = usize;

#[get("/<id>")]
pub async fn get_block(id: ID, node: StateNode<'_>) -> Option<Json<Block>> {
    match node.lock().await.blockchain.chain.get(id) {
        Some(block) => Some(Json(block.clone())),
        None => None
    }
}

#[get("/chain")]
pub async fn get_chain(node: StateNode<'_>) -> Value {
    let node = node.lock().await;
    json!(node.blockchain.chain)
    //Json(node.blockchain.chain.clone())
}

#[post("/mine")]
pub async fn mine(node: StateNode<'_>) -> Value {
    let mut node = node.lock().await;
    match node.blockchain.mine_block() {
        Ok(..) => json!(node.blockchain.chain.get(node.blockchain.chain.len() - 1)),
        Err(m) => json!({"error": m}),
    }
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

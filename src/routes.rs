use crate::AppState;
use crate::{Block, Transaction};
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MineResult {
    message: String,
    block: Option<Block>,
    funds: i32,
}

pub async fn get_chain(data: web::Data<AppState>) -> Result<impl Responder> {
    let node = data.node.lock().unwrap();
    Ok(web::Json(node.blockchain.chain.clone()))
}

#[get("/balance")]
pub async fn get_balance(data: web::Data<AppState>) -> impl Responder {
    let node = data.node.lock().unwrap();
    HttpResponse::Ok().body(format!("Available fund: {}", node.blockchain.get_balance()))
}

#[post("/mine")]
pub async fn mine_block(data: web::Data<AppState>) -> Result<impl Responder> {
    let mut node = data.node.lock().unwrap();
    let res = node.blockchain.mine_block();
    match res {
        Ok(block) => {
            return Ok(web::Json(MineResult {
                message: "New block has been mined".to_string(),
                block: Some(block),
                funds: node.blockchain.get_balance(),
            }))
        }
        Err(_) => {
            return Ok(web::Json(MineResult {
                message: "Error".to_string(),
                block: None,
                funds: 0,
            }))
        }
    }
}

#[post("/transaction")]
pub async fn add_transaction(
    data: web::Data<AppState>,
    tr: web::Json<Transaction>,
) -> Result<impl Responder> {
    let mut node = data.node.lock().unwrap();
    let signature =
        node.wallet
            .sign_transaction(&node.blockchain.hosting_node_id, &tr.recipient, &tr.amount);
    node.blockchain
        .add_transaction(tr.recipient.clone(), tr.amount.clone(), &signature);
    Ok(web::Json(Transaction::new(
        &tr.sender,
        &tr.recipient,
        &tr.amount,
        &signature,
    )))
}

#[get("/transactions")]
pub async fn get_open_transactions(data: web::Data<AppState>) -> Result<impl Responder> {
    let node = data.node.lock().unwrap();
    Ok(web::Json(node.blockchain.transactions.clone()))
}

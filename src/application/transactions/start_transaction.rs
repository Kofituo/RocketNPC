use rand::distributions::{Distribution, Uniform};
use rocket::serde::json::Json;

use crate::model::api_error::APIError;
use crate::model::api_response;
use crate::model::transactions::start_transaction;

const FILE: &str = "application/start_transaction.rs";

pub fn start_transaction(
    start_transaction: &start_transaction::StartTransaction,
) -> Result<Json<api_response::ApiResponse>, APIError> {
    const METHOD: &str = "start_transaction";
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..999999999);

    let transaction_id = die.sample(&mut rng);

    let response: api_response::ApiResponse = api_response::ApiResponse::new(
        String::from(""),
        transaction_id.to_string(),
        String::from("Transaction started successfully"),
        0,
    );
    println!("{}", "Start Transaction Sucess.");
    Ok(Json(response))
}

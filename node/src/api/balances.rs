use actix_web::{HttpResponse, Scope, get, web};
use serde::Serialize;

use crate::{api::error::ApiError, Account, State};

#[get("/list")]
async fn list_balances() -> Result<HttpResponse, ApiError> {
  let state = State::new_from_disk()?;
  let balances = state.balances.iter().map(|(acc, amt)| {
    BalanceResponse{
      account: acc.clone(),
      amount: *amt
    }
  }).collect::<Vec<_>>();

  let response = StateResponse{
    balances,
    latest_blockhash: state.latest_blockhash.to_hex()
  };

  Ok(HttpResponse::Ok().json(response))
}

pub fn router() -> Scope {
    web::scope("/balances").service(list_balances)
}

#[derive(Serialize)]
pub struct BalanceResponse{
  account: Account,
  amount: u64
}

#[derive(Serialize)]
pub struct StateResponse{
  latest_blockhash: String,
  balances: Vec<BalanceResponse>
}
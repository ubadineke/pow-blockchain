use actix_web::{post, web, HttpResponse, Scope};

use crate::{api::error::ApiError, State, Tx};

#[post("/add")]
async fn add_tx(payload: web::Json<Tx>) -> Result<HttpResponse, ApiError> {
   let mut state = State::new_from_disk()?;
  let tx = payload.0;
   state.add(tx)?;
   state.persist()?;

  Ok(HttpResponse::Ok().body("TX successfully added to ledger"))
}

pub fn router() -> Scope {
    web::scope("/tx").service(add_tx)
}

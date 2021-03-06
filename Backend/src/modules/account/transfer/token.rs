use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::dto::Failure;
use crate::modules::account::dto::{CreateToken, ProlongToken};
use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Token;

#[openapi]
#[post("/token", format = "application/json", data = "<params>")]
pub fn create_token(me: State<Account>, auth: Authenticate, params: Json<CreateToken>) -> Result<Json<APIToken>, Failure>
{
  me.create_token(&params.purpose, auth.0, params.exp_date)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[openapi]
#[get("/token")]
pub fn get_tokens(me: State<Account>, auth: Authenticate) -> Result<Json<Vec<APIToken>>, Failure> {
  Ok(Json(me.get_all_token(auth.0)))
}

#[openapi]
#[delete("/token", format = "application/json", data = "<token_id>")]
pub fn delete_token(me: State<Account>, auth: Authenticate, token_id: Json<u32>) -> Result<(), Failure>
{
  me.delete_token(token_id.0, auth.0)
}

#[openapi]
#[post("/token/prolong", format = "application/json", data = "<params>")]
pub fn prolong_token(me: State<Account>, auth: Authenticate, params: Json<ProlongToken>) -> Result<Json<APIToken>, Failure>
{
  me.prolong_token_by_str(params.token.clone(), auth.0, params.days)
    .and_then(|api_token| Ok(Json(api_token)))
}
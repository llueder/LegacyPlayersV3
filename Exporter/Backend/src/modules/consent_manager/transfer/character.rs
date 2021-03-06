use rocket::State;

use crate::dto::Failure;
use crate::modules::consent_manager::tools::{CharacterConsent, ManagerFrontend};
use crate::modules::ConsentManager;
use crate::modules::consent_manager::guard::Authenticate;
use crate::modules::consent_manager::domain_value::CharacterWithConsent;
use rocket_contrib::json::Json;

#[get("/character")]
pub fn get_characters(me: State<ConsentManager>, auth: Authenticate) -> Json<Vec<CharacterWithConsent>> {
  Json(me.get_characters(auth.0))
}

#[post("/character/<character_id>")]
pub fn give_consent(me: State<ConsentManager>, _auth: Authenticate, character_id: u32) -> Result<(), Failure>
{
  me.give_consent(character_id)
}

#[delete("/character/<character_id>")]
pub fn withdraw_consent(me: State<ConsentManager>, _auth: Authenticate, character_id: u32) -> Result<(), Failure>
{
  me.withdraw_consent(character_id)
}
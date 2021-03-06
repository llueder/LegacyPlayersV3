use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::StatType;
use crate::modules::data::tools::RetrieveStatType;

#[openapi]
#[get("/stat_type/<id>")]
pub fn get_stat_type(me: State<Data>, id: u8) -> Option<Json<StatType>>
{
  me.get_stat_type(id)
    .and_then(|stat_type| Some(Json(stat_type)))
}

#[openapi]
#[get("/stat_type")]
pub fn get_all_stat_types(me: State<Data>) -> Json<Vec<StatType>>
{
  Json(me.get_all_stat_types())
}
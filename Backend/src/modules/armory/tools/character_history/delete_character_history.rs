use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::dto::ArmoryFailure;
use crate::modules::armory::tools::GetCharacterHistory;

pub trait DeleteCharacterHistory {
  fn delete_character_history(&self, character_history_id: u32) -> Result<(), ArmoryFailure>;
}

impl DeleteCharacterHistory for Armory {
  fn delete_character_history(&self, character_history_id: u32) -> Result<(), ArmoryFailure> {
    let character_history_res = self.get_character_history(character_history_id);
    if character_history_res.is_err() {
      return Err(ArmoryFailure::InvalidInput);
    }
    let character_history = character_history_res.unwrap();

    let mut characters = self.characters.write().unwrap();
    if self.db_main.execute_wparams("DELETE FROM armory_character_history WHERE id=:id", params!(
      "id" => character_history_id
    )) {
      let mut character = characters.get_mut(&character_history.character_id).unwrap();
      let hm = character.history_moments.iter().find(|history_moment| history_moment.id == character_history_id).unwrap().clone();
      character.history_moments.remove_item(&hm);
      if character.last_update.contains(&character_history) {
        let last_id = character.history_moments.last();
        if last_id.is_some() {
          character.last_update = self.get_character_history(last_id.unwrap().id).ok();
        }
      }
      return Ok(());
    }
    Err(ArmoryFailure::Database("delete_character_history".to_owned()))
  }
}
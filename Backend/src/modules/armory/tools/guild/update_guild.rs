use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::tools::GetGuild;
use crate::modules::armory::dto::ArmoryFailure;

pub trait UpdateGuild {
  fn update_guild_name(&self, server_id: u32, uid: u64, guild_name: String) -> Result<(), ArmoryFailure>;
}

impl UpdateGuild for Armory {
  fn update_guild_name(&self, server_id: u32, uid: u64, guild_name: String) -> Result<(), ArmoryFailure> {
    let guild_id = self.get_guild_id_by_uid(server_id, uid).unwrap();
    let mut guilds = self.guilds.write().unwrap();
    if self.db_main.execute_wparams("UPDATE armory_guild SET guild_name=:guild_name WHERE server_id=:server_id AND server_uid=:server_uid", params!(
      "server_id" => server_id,
      "server_uid" => uid,
      "guild_name" => guild_name.clone()
    )) {
      let mut guild = guilds.get_mut(&guild_id).unwrap();
      guild.name = guild_name.to_owned();
      return Ok(());
    }
    Err(ArmoryFailure::InvalidInput)
  }
}
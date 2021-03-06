use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemQuality;

pub trait RetrieveItemQuality {
  fn get_item_quality(&self, id: u8) -> Option<ItemQuality>;
  fn get_all_item_qualities(&self) -> Vec<ItemQuality>;
}

impl RetrieveItemQuality for Data {
  fn get_item_quality(&self, id: u8) -> Option<ItemQuality> {
    self.item_qualities.get(&id)
      .and_then(|item_quality| Some(item_quality.clone()))
  }

  fn get_all_item_qualities(&self) -> Vec<ItemQuality> {
    self.item_qualities.iter().map(|(_, item_quality)| item_quality.clone()).collect()
  }
}

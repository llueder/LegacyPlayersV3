use crate::modules::data::Data;
use crate::modules::data::domain_value::HeroClass;

pub trait RetrieveHeroClass {
  fn get_hero_class(&self, id: u8) -> Option<HeroClass>;
  fn get_all_hero_classes(&self) -> Vec<HeroClass>;
}

impl RetrieveHeroClass for Data {
  fn get_hero_class(&self, id: u8) -> Option<HeroClass> {
    self.hero_classes.get(&id)
      .and_then(|hero_class| Some(hero_class.clone()))
  }

  fn get_all_hero_classes(&self) -> Vec<HeroClass> {
    self.hero_classes.iter().map(|(_, hero_class)| hero_class.clone()).collect()
  }
}

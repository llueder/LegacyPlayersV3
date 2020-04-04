use crate::modules::live_data_processor::dto::Message;
use crate::modules::live_data_processor::domain_value::Event;

pub struct Server {
  pub non_committed_messages: Vec<Message>,
  pub committed_events: Vec<Event>
}

impl Default for Server {
  fn default() -> Self {
    Server {
      non_committed_messages: Vec::new(), // TODO: Smarter Data structure that splits per unit
      committed_events: Vec::new()
    }
  }
}

impl Server {
  pub fn init(mut self) -> Self {
    self
  }
}
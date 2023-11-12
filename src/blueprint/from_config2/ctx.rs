use crate::blueprint::from_config2::Ctx;
use crate::config::{Config, Type};

impl Ctx for Config {
  fn is_known_type(&self, type_name: &str) -> bool {
    todo!()
  }

  fn find_type(&self, type_name: &str) -> Option<&Type> {
    self.graphql.find_type(type_name)
  }
}

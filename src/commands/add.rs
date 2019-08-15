use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::config::{self, Alias};

pub fn add(name: &str, command: &str, condition: Option<&str>) -> Result<(), failure::Error> {
  let cfg = config::load()?;
  if check_already_registered(name, &cfg) {
    let msg = failure::err_msg(format!("alias `{}` is already registered", name));
    return Err(msg);
  }

  let mut cfg: BTreeMap<String, Alias> = BTreeMap::from_iter(cfg.into_iter());
  cfg.insert(name.to_string(), create_alias(command, condition));
  config::save(cfg)?;

  return Ok(());
}

fn check_already_registered(name: &str, cfg: &BTreeMap<String, Alias>) -> bool {
  return match cfg.get(name) {
    Some(_) => true,
    None => false,
  };
}

fn create_alias(command: &str, condition: Option<&str>) -> Alias {
  let condition = match condition {
    Some(value) => Some(value.to_string()),
    None => None,
  };

  return Alias {
    command: command.to_string(),
    condition,
  };
}

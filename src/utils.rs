pub fn bool_to_string(value: Option<bool>) -> Option<String> {
    Some(match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "false",
    }
    .to_string())
}

pub fn int_to_string(value: Option<i32>) -> Option<String> {
  match value {
      Some(v) => Some(v.to_string()),
      None => None
  }
}


use polywrap_wasm_rs::{Map};

pub fn bool_to_string(value: Option<bool>) -> Option<String> {
    match value {
        Some(true) => Some("true".to_string()),
        Some(false) => None,
        None => None,
    }
}

pub fn int_to_string(value: Option<i32>) -> Option<String> {
  match value {
      Some(v) => Some(v.to_string()),
      None => None
  }
}

pub trait AddOptVal<T> {
    fn add(&mut self, key: &str, value: Option<T>);
}

impl<T> AddOptVal<T> for Map<String, T> {
    fn add(&mut self, key: &str, value: Option<T>) {
        if let Some(val) = value {
            self.insert(key.to_string(), val);
        }
    }
}

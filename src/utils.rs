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

#[derive(Copy, Clone)]
pub enum MaybeOption<'a, T> where T:Clone {
    OptVal(Option<&'a T>),
    Val(&'a T),
}

pub trait MaybeOptionTrait<T> where T:Clone {
    fn get_value(&self) -> MaybeOption<T>;
}

impl<T: Clone> MaybeOptionTrait<T> for Option<T> {
    fn get_value(&self) -> MaybeOption<T> {
        MaybeOption::OptVal(self.as_ref())
    }
}

impl<T: Clone> MaybeOptionTrait<T> for T {
    fn get_value(&self) -> MaybeOption<T> {
        MaybeOption::Val(self)
    }
}

pub trait AddOptVal<T: Clone, V: MaybeOptionTrait<T>> {
    fn add(&mut self, key: &str, value: V);
}

impl<T: Clone, V: MaybeOptionTrait<T>> AddOptVal<T, V> for Map<String, T> {
    fn add(&mut self, key: &str, value: V) {
        let maybe_opt_val: MaybeOption<T> = value.get_value();
        match maybe_opt_val {
            MaybeOption::OptVal(val) => {
                if let Some(v) = val {
                    self.insert(key.to_string(), v.to_owned());
                }
            },
            MaybeOption::Val(val) => {
                self.insert(key.to_string(), val.to_owned());
            }
        }
        // if let Some(val) = value {
        //     self.insert(key.to_string(), val);
        // }
    }
}

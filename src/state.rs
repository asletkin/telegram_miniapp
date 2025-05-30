use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static VOTES: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(|| Mutex::new(HashMap::new()));


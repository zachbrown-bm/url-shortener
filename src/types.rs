use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type UrlStore = Arc<RwLock<HashMap<String, String>>>;
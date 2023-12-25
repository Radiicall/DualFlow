use serde::{Deserialize, Serialize};

use crate::objects::Trigger;

#[derive(Deserialize, Serialize)]
pub struct AppSettings {
    pub left_trigger: Trigger,
    pub right_trigger: Trigger,
}

#[derive(Deserialize, Serialize)]
pub struct Apps {
    pub apps: Vec<String>,
}

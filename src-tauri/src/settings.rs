use serde::{Deserialize, Serialize};

#[cfg(not(feature = "fake"))]
use crate::objects::Trigger;
#[cfg(feature = "fake")]
use crate::fake::Trigger;

#[derive(Deserialize, Serialize)]
pub struct AppSettings {
    pub left_trigger: Trigger,
    pub right_trigger: Trigger,
}

#[derive(Deserialize, Serialize)]
pub struct Apps {
    pub apps: Vec<String>,
}

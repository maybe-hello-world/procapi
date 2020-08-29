use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct InputData {
    pub img64: String
}
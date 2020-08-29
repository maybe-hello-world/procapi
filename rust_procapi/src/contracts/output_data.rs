use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct OutputData<'a> {
    pub result_class: &'a str
}
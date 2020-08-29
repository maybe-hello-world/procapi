use std::collections::HashMap;
use std::error::Error;

use crate::preprocessors::traits::Processor;

#[derive(Debug, Clone)]
pub(crate) struct OutputPreprocessor {
    _map: HashMap<&'static str, &'static str>
}

impl Processor<&str, &str> for OutputPreprocessor {
    fn preprocess_data(&self, data: &str) -> Result<&'static str, Box<dyn Error>> {
        Ok(self._map.get(data).unwrap_or(&"unknown"))
    }

    fn new() -> OutputPreprocessor {
        OutputPreprocessor {
            _map: vec![
                ("0", "cat"),
                ("1", "dog")
            ].into_iter().collect()
        }
    }
}
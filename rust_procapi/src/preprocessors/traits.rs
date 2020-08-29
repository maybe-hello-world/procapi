use std::error::Error;

pub trait Processor<T, TR> {
    fn preprocess_data(&self, data: T) -> Result<TR, Box<dyn Error>>;
    fn new() -> Self;
}
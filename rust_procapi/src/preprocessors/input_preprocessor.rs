extern crate base64;
extern crate image;

use std::error::Error;

use base64::{decode, encode};
use image::imageops::{FilterType::Nearest, grayscale, resize};
use image::load_from_memory;

use crate::contracts::input_data::InputData;
use crate::preprocessors::traits::Processor;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

#[derive(Clone, Debug)]
pub(crate) struct InputPreprocessor {}

impl Processor<InputData, String> for InputPreprocessor {
    fn preprocess_data(&self, data: InputData) -> Result<String, Box<dyn Error>> {
        let bytes = decode(data.img64)?;

        let img = load_from_memory(&bytes)?;
        let img = resize(&img, WIDTH, HEIGHT, Nearest);
        let img = grayscale(&img);

        Ok(encode(img.to_vec()))
    }

    fn new() -> InputPreprocessor {
        InputPreprocessor {}
    }
}
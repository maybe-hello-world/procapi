use actix::Addr;
use actix_web::{HttpResponse, Responder, web};
use redis::RedisError;
use serde::Deserialize;

use crate::contracts::input_data::InputData;
use crate::contracts::output_data::OutputData;
use crate::controllers::prediction_controller::controller::{GetCommand, PredictionActor, PredictionPreprocessor};
use crate::preprocessors::traits::Processor;

#[derive(Deserialize)]
pub struct ResultRequest {
    pub id: String
}

pub(crate) async fn short(
    data: web::Json<InputData>,
    actor: web::Data<Addr<PredictionActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub(crate) async fn long(
    data: web::Json<InputData>,
    actor: web::Data<Addr<PredictionActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


pub(crate) async fn result(
    info: web::Query<ResultRequest>,
    actor: web::Data<Addr<PredictionActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {
    // ask for the result
    let result = actor.send(GetCommand { key: info.id.clone() }).await;

    // check actor mailbox error
    let result: Result<Option<String>, redis::RedisError> = match result {
        Err(e) => return HttpResponse::InternalServerError().body("actor mailbox error"),
        Ok(x) => x
    };

    // check that redis connection isn't closed
    let result = match result {
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        Ok(x) => x
    };

    // return valid answer
    return match result {
        Some(value) => {
            let answer = preprocessors.output_preprocessor.preprocess_data(&value).unwrap();
            HttpResponse::Ok().json( OutputData {
                result_class: answer
            })
        },
        None => HttpResponse::NoContent().body("")
    }
}
use actix::Addr;
use actix_web::{HttpResponse, Responder, web};
use redis::RedisError;
use serde::{Deserialize, Serialize};

use crate::contracts::input_data::InputData;
use crate::contracts::output_data::OutputData;
use crate::contracts::backend_message::{BackendMessage, BackendMessageType};
use crate::controllers::prediction_controller::controller::{RedisGetCommand, RedisActor, PredictionPreprocessor, RabbitActor, RabbitLongSendCommand, RabbitShortSendCommand};
use crate::preprocessors::traits::Processor;
use std::error::Error;
use uuid;
use either::Either;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ResultRequest {
    pub id: String
}

async fn send_to_rabbit(
    actor: Addr<RabbitActor>,
    data: InputData,
    message_type: BackendMessageType,
    preprocessors: &PredictionPreprocessor
) -> Result<Either<OutputData, Uuid>> {
    let data = preprocessors.input_preprocessor.preprocess_data(data)?;
    let id = match message_type {
        BackendMessageType::Long => Some(uuid::Uuid::new_v4()),
        BackendMessageType::Short => None
    };

    let backend_mesage = BackendMessage { message_type, data, id: id.clone() };
    let bytes = serde_json::to_string(&backend_mesage)?.into_bytes();

    if message_type == BackendMessageType::Short {
        let result:  = actor.send(RabbitShortSendCommand { payload: bytes }).await?;
        let output_data =
    } else {
        actor.send(RabbitLongSendCommand { payload: bytes }).await?;
        return Ok(Either::Right(id.unwrap()))
    }

}

pub(crate) async fn short(
    data: web::Json<InputData>,
    actor: web::Data<Addr<RedisActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub(crate) async fn long(
    data: web::Json<InputData>,
    actor: web::Data<Addr<RabbitActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {


    HttpResponse::Ok().body("Hello world!")
}


pub(crate) async fn result(
    info: web::Query<ResultRequest>,
    actor: web::Data<Addr<RedisActor>>,
    preprocessors: web::Data<PredictionPreprocessor>
) -> impl Responder {
    // ask for the result
    let result = actor.send(RedisGetCommand { key: info.id.clone() }).await;

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
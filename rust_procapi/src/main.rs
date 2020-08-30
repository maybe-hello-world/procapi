extern crate lazy_static;

use std::io::ErrorKind;

use actix::{Actor, SyncArbiter};
use actix_web::{App, HttpServer, web};

use crate::controllers::prediction_controller::controller::{RedisActor, PredictionPreprocessor, RabbitActor};
use crate::controllers::prediction_controller::methods::{long, result, short};

mod preprocessors;
mod controllers;
mod contracts;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let redis_actor = match RedisActor::new() {
        Err(e) => return Err(std::io::Error::new(ErrorKind::Other, e)),
        Ok(x) => x
    };
    let redis_addr = redis_actor.start();

    let rabbit_actor = match RabbitActor::new() {
        Err(e) => return Err(std::io::Error::new(ErrorKind::Other, e)),
        Ok(x) => x
    };
    let rabbit_addr = rabbit_actor.start();


    HttpServer::new(move || {
        // thread local context
        let preprocessors = PredictionPreprocessor::new();

        App::new()
            .service(
                web::scope("/prediction")
                    .data(redis_addr.clone())
                    .data(rabbit_addr.clone())
                    .data(preprocessors)
                    .route("/long", web::get().to(long))
                    .route("/short", web::post().to(short))
                    .route("/result", web::get().to(result))
            )
    })
        .bind("0.0.0.0:5001")?
        .run()
        .await

    // TODO: add logging everywhere
    // TODO: rust-clippy
}
extern crate lazy_static;

use std::io::ErrorKind;

use actix::Actor;
use actix_web::{App, HttpServer, web};

use crate::controllers::prediction_controller::controller::{PredictionActor, PredictionPreprocessor};
use crate::controllers::prediction_controller::methods::{long, result, short};

mod preprocessors;
mod controllers;
mod contracts;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let prediction_state_actor = match PredictionActor::new() {
        Err(e) => return Err(std::io::Error::new(ErrorKind::Other, e)),
        Ok(x) => x
    };
    let addr = prediction_state_actor.start();

    HttpServer::new(move || {
        // thread local context
        let preprocessors = PredictionPreprocessor::new();

        App::new()
            .service(
                web::scope("/prediction")
                    .data(addr.clone())
                    .data(preprocessors)
                    .route("/long", web::get().to(long))
                    .route("/short", web::post().to(short))
                    .route("/result", web::get().to(result))
            )
    })
        .bind("0.0.0.0:5001")?
        .run()
        .await
}
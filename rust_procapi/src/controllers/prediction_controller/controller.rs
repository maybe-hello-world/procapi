use std::env;

use actix::prelude::*;
use redis;
use redis::{AsyncCommands, RedisError};
use redis::aio::MultiplexedConnection;
use serde::Deserialize;

use crate::preprocessors::input_preprocessor::InputPreprocessor;
use crate::preprocessors::output_preprocessor::OutputPreprocessor;
use crate::preprocessors::traits::Processor;

pub(crate) struct PredictionPreprocessor {
    pub(crate) input_preprocessor: InputPreprocessor,
    pub(crate) output_preprocessor: OutputPreprocessor,
}

impl PredictionPreprocessor {
    pub fn new() -> Self {
        PredictionPreprocessor {
            input_preprocessor: InputPreprocessor::new(),
            output_preprocessor: OutputPreprocessor::new(),
        }
    }
}


pub(crate) struct PredictionActor {
    redis_connection: MultiplexedConnection,
    rabbit_connection: String,
}

impl PredictionActor {
    fn create_redis_conn() -> Result<MultiplexedConnection, RedisError> {
        let env_str = env::var("REDIS_CONNECTION_STRING").unwrap_or(String::from("redis://localhost:6379"));
        let client = redis::Client::open(env_str)?;
        let con = futures::executor::block_on(client.get_multiplexed_async_std_connection())?;
        Ok(con)
    }

    pub fn new() -> Result<Self, RedisError> {
        // redis
        let redis_conn = PredictionActor::create_redis_conn()?;

        // rabbitmq
        let login = env::var("RABBITMQ_USERNAME").unwrap_or(String::from("guest"));
        let pass = env::var("RABBITMQ_PASSWORD").unwrap_or(String::from("guest"));
        let vhost = env::var("RABBITMQ_VIRTUALHOST").unwrap_or(String::from("/"));
        let hostname = env::var("RABBITMQ_HOSTNAME").unwrap_or(String::from("localhost"));
        let port = env::var("RABBIT_MQ_PORT").ok().and_then(|x| x.parse::<u32>().ok()).unwrap_or(5672);

        Ok(PredictionActor {
            redis_connection: redis_conn,
            rabbit_connection: "".to_string(),
        })
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub(crate) struct GetCommand {
    pub(crate) key: String
}

impl Handler<GetCommand> for PredictionActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, msg: GetCommand, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.redis_connection.clone();
        let fut = async move {
            conn.get(&msg.key).await
        };
        Box::pin(fut)
    }
}

impl Actor for PredictionActor {
    type Context = Context<Self>;
}
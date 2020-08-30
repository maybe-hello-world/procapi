use std::env;

use actix::prelude::*;
use lapin::{Connection, ConnectionProperties, Error, Channel, BasicProperties, Consumer};
use lapin::uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use redis;
use redis::{AsyncCommands, RedisError};
use redis::aio::MultiplexedConnection;

use crate::preprocessors::input_preprocessor::InputPreprocessor;
use crate::preprocessors::output_preprocessor::OutputPreprocessor;
use crate::preprocessors::traits::Processor;
use lapin::options::{QueueDeclareOptions, BasicPublishOptions, BasicConsumeOptions};
use lapin::publisher_confirm::PublisherConfirm;
use lapin::types::{ShortString, FieldTable};
use futures::StreamExt;

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


pub(crate) struct RedisActor {
    redis_connection: MultiplexedConnection,
}

impl RedisActor {
    pub fn new() -> Result<Self, RedisError> {
        let env_str = env::var("REDIS_CONNECTION_STRING").unwrap_or(String::from("redis://localhost:6379"));
        let client = redis::Client::open(env_str)?;
        let redis_conn = futures::executor::block_on(client.get_multiplexed_async_std_connection())?;

        Ok(RedisActor { redis_connection: redis_conn })
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub(crate) struct RedisGetCommand {
    pub(crate) key: String
}

impl Handler<RedisGetCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, msg: RedisGetCommand, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.redis_connection.clone();
        let fut = async move {
            conn.get(&msg.key).await
        };
        Box::pin(fut)
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

pub(crate) struct RabbitActor {
    channel: Channel,
    queue_name: String
}

impl RabbitActor {
    pub(crate) fn new() -> Result<Self, Error> {
        let username = env::var("RABBITMQ_USERNAME").unwrap_or(String::from("guest"));
        let password = env::var("RABBITMQ_PASSWORD").unwrap_or(String::from("guest"));
        let vhost = env::var("RABBITMQ_VIRTUALHOST").unwrap_or(String::from("/"));
        let host = env::var("RABBITMQ_HOSTNAME").unwrap_or(String::from("localhost"));
        let port = env::var("RABBIT_MQ_PORT").ok().and_then(|x| x.parse::<u16>().ok()).unwrap_or(5672);

        let uri: AMQPUri = AMQPUri {
            scheme: Default::default(),
            authority: AMQPAuthority {
                userinfo: AMQPUserInfo { username, password },
                host,
                port,
            },
            vhost,
            query: Default::default(),
        };
        let conprops = ConnectionProperties::default();
        let connection: Connection = futures::executor::block_on(Connection::connect_uri(uri, conprops))?;
        let channel: Channel = futures::executor::block_on(connection.create_channel())?;

        let queue_name = env::var("RABBITMQ_QUEUENAME").unwrap_or(String::from("rpc_queue"));
        channel.queue_declare(&queue_name, QueueDeclareOptions::default(), FieldTable::default());

        Ok(Self { channel, queue_name })
    }

    async fn handle_short_message(channel: Channel, msg: RabbitShortSendCommand, queue_name: String) -> Result<Consumer, Error> {
        // https://github.com/actix/actix/issues/308
        let properties = BasicProperties::default().with_reply_to(ShortString::from("amq.rabbitmq.reply-to"));
        channel.basic_publish(
            "",
            &queue_name,
            BasicPublishOptions::default(),
            msg.payload,
            properties
        ).await?;

        channel.basic_consume(
            "amq.rabbitmq.reply-to",
            "direct_consumer",
            BasicConsumeOptions {
                no_local: false,
                no_ack: true,
                exclusive: false,
                nowait: false
            },
            FieldTable::default()
        )
            .await
            .unwrap().skip_while(|x| x.is_err())
            .next()

    }

    async fn handle_long_message(channel: Channel, msg: RabbitLongSendCommand, queue_name: String) -> Result<(), Error> {
        // https://github.com/actix/actix/issues/308
        return match channel.basic_publish(
            "",
            &queue_name,
            BasicPublishOptions::default(),
            msg.payload,
            BasicProperties::default()
        ).await {
            Ok(x) => Ok(()),
            Err(e) => Err(e)
        }
    }
}

impl Actor for RabbitActor {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Consumer, lapin::Error>")]
pub(crate) struct RabbitShortSendCommand {
    pub(crate) payload: Vec<u8>
}

impl Handler<RabbitShortSendCommand> for RabbitActor {
    type Result = ResponseActFuture<Self, Result<Consumer, lapin::Error>>;

    fn handle(&mut self, msg: RabbitShortSendCommand, _: &mut Self::Context) -> Self::Result {
        let channel = self.channel.clone();
        Box::pin(actix::fut::wrap_future(RabbitActor::handle_short_message(channel, msg, self.queue_name.clone())))
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), lapin::Error>")]
pub(crate) struct RabbitLongSendCommand {
    pub(crate) payload: Vec<u8>
}

impl Handler<RabbitLongSendCommand> for RabbitActor {
    type Result = ResponseActFuture<Self, Result<(), lapin::Error>>;

    fn handle(&mut self, msg: RabbitLongSendCommand, _: &mut Self::Context) -> Self::Result {
        let channel = self.channel.clone();
        Box::pin(actix::fut::wrap_future(RabbitActor::handle_long_message(channel, msg, self.queue_name.clone())))
    }
}
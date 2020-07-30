import dataclasses
import json
import os
from asyncio import Future
from typing import Union
from uuid import uuid4

import redis
import aio_pika
from aio_pika import Connection, Channel, Message, Queue
from fastapi import APIRouter, Response
from redis import Redis

from Contracts.BackendMessage import BackendMessageType, BackendMessage
from Contracts.InputData import InputData
from Contracts.OutputData import OutputData
from Preprocessors.InputPreprocessor import InputPreprocessor
from Preprocessors.OutputPreprocessor import OutputPreprocessor
from Utils.parsers import try_parse_int

input_preprocessor = InputPreprocessor()
output_preprocessor = OutputPreprocessor()

connection: Connection
channel: Channel
routing_key: str
queue: Queue
DEFAULT_TIMEOUT: int
redis_host: Redis

router = APIRouter()


@router.on_event("startup")
async def startup():
    global connection, channel, routing_key, queue, redis_host, DEFAULT_TIMEOUT

    login = os.environ.get("RABBITMQ_USERNAME", "guest")
    password = os.environ.get("RABBITMQ_PASSWORD", "guest")
    virtual_host = os.environ.get("RABBITMQ_VIRTUALHOST", "/")
    hostname = os.environ.get("RABBITMQ_HOSTNAME", "localhost")
    port = os.environ.get("RABBITMQ_PORT", "5672")
    if (port := try_parse_int(port)) is None:
        port = 5672
    routing_key = os.environ.get("RABBITMQ_QUEUENAME", "rpc_queue")
    redis_connection = os.environ.get("REDIS_CONNECTION_STRING", "redis://localhost:6379")
    DEFAULT_TIMEOUT = 10

    # open connection to rabbitmq
    connection = await aio_pika.connect_robust(
        host=hostname,
        port=port,
        virtualhost=virtual_host,
        login=login,
        password=password
    )
    channel = await connection.channel()
    queue = await channel.declare_queue("amq.rabbitmq.reply-to")

    # open connection to redis
    redis_host = redis.Redis.from_url(redis_connection)


@router.on_event("shutdown")
async def shutdown():
    global channel, connection
    await channel.close()
    await connection.close()


async def send_to_rabbit(mtype: BackendMessageType, data: InputData) -> Union[OutputData, str]:
    global routing_key, channel

    preprocessed_data = input_preprocessor.preprocess_data(data)
    mid = uuid4()

    backend_message = BackendMessage(
        MessageType=mtype,
        Data=preprocessed_data,
        Id=mid if mtype == BackendMessageType.Long else None
    )

    message_body = json.dumps(dataclasses.asdict(backend_message), default=str).encode("utf-8")

    if mtype == BackendMessageType.Short:
        q_result = Future()
        await queue.consume(no_ack=True, timeout=DEFAULT_TIMEOUT, callback=q_result.set_result)

        await channel.default_exchange.publish(
            message=Message(body=message_body, reply_to="amq.rabbitmq.reply-to"),
            routing_key=routing_key
        )

        q_result = await q_result
        return output_preprocessor.preprocess_data(q_result.body.decode("utf-8"))
    else:
        await channel.default_exchange.publish(message=Message(body=message_body), routing_key=routing_key)
        return str(mid)


@router.post("/short", response_model=OutputData)
async def short(data: InputData):
    req_result = await send_to_rabbit(BackendMessageType.Short, data)
    return req_result


@router.post("/long", response_model=str)
async def long(data: InputData):
    req_result = await send_to_rabbit(mtype=BackendMessageType.Long, data=data)
    return req_result


@router.get("/result", response_model=OutputData, status_code=200)
async def result(id: str, response: Response):
    result_id = redis_host.get(id)
    if result_id is None:
        response.status_code = 204
        return
    return output_preprocessor.preprocess_data(result_id)

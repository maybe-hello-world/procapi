using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Configuration;
using ProcAPI.Contracts;
using ProcAPI.Preprocessors;
using ProcAPI.Utils;
using RabbitMQ.Client;
using RabbitMQ.Client.Events;
using System;
using System.Threading.Tasks;
using StackExchange.Redis;

namespace ProcAPI.Controllers
{
    [ApiController]
    [Route("[controller]/[action]")]
    public class PredictionController : ControllerBase
    {
        private readonly IPreprocessor<InputData, string> _inputPreprocessor;
        private readonly IPreprocessor<string, OutputData> _outputPreprocessor;

        private readonly string _routingKey;
        private const string ReplyToQueue = "amq.rabbitmq.reply-to";
        private readonly IConnection _conn;
        private readonly IModel _channel;

        private readonly ConnectionMultiplexer _redisConnections;

        public PredictionController(IConfiguration configuration)
        {
            _inputPreprocessor = new InputPreprocessor();
            _outputPreprocessor = new OutputPreprocessor();

            // Initiate RabbitMQ connection
            _conn = new ConnectionFactory
            {
                UserName = configuration["RABBITMQ_USERNAME"] ?? "guest",
                Password = configuration["RABBITMQ_PASSWORD"] ?? "guest",
                VirtualHost = configuration["RABBITMQ_VIRTUALHOST"] ?? "/",
                HostName = configuration["RABBITMQ_HOSTNAME"] ?? "localhost",
                Port = int.TryParse(configuration["RABBITMQ_PORT"] ?? "5672", out var result) ? result : 5672
            }.CreateConnection();

            _channel = _conn.CreateModel();
            _routingKey = configuration["RABBITMQ_QUEUENAME"] ?? "rpc_queue";


            // Initiate Redis connection
            _redisConnections = ConnectionMultiplexer.Connect(configuration["REDIS_HOSTS"] ?? "localhost");
        }


        private async Task<string> SendMessageAsync(string routingKey, byte[] messageBodyBytes)
        {
            var tcs = new TaskCompletionSource<string>();

            var consumer = new EventingBasicConsumer(_channel);
            consumer.Received += (ch, ea) =>
            {
                var body = ea.Body.ToArray();
                tcs.SetResult(System.Text.Encoding.UTF8.GetString(body));
            };
            _channel.BasicConsume(ReplyToQueue, true, consumer);

            var props = _channel.CreateBasicProperties();
            props.ReplyTo = ReplyToQueue;
            _channel.BasicPublish("", routingKey, props, messageBodyBytes);

            return await tcs.Task;
        }

        private async Task<Either<OutputData, Guid>> SendToRabbit(BackendMessageType type, InputData data)
        {
            var preprocessedData = _inputPreprocessor.PreprocessData(data);
            var id = Guid.NewGuid();
            var backendMessage = new BackendMessage
            {
                MessageType = type,
                Data = preprocessedData,
                Id = type == BackendMessageType.Short ? (Guid?) null : id
            };

            var messageBodyBytes = System.Text.Encoding.UTF8.GetBytes(
                Newtonsoft.Json.JsonConvert.SerializeObject(backendMessage)
            );

            if (type == BackendMessageType.Short)
            {
                var res = await SendMessageAsync(_routingKey, messageBodyBytes);
                var outputData = _outputPreprocessor.PreprocessData(res);
                return outputData;
            }
            else
            {
                _channel.BasicPublish("", _routingKey, null, messageBodyBytes);
                return id;
            }
        }

        /// <summary>
        /// Preprocess input data, send the task to RabbitMQ and wait for data immediately to return
        /// </summary>
        /// <param name="data">input data to process</param>
        /// <returns>output data</returns>
        [HttpPost]
        public async Task<ActionResult> Short([FromBody] InputData data) =>
            (await SendToRabbit(BackendMessageType.Short, data))
            .Match(
                outputData => new ContentResult
                {
                    Content = Newtonsoft.Json.JsonConvert.SerializeObject(outputData),
                    ContentType = "application/json",
                    StatusCode = 200
                },
                id => throw new Exception("Internal error: unexpected message result from SendToRabbit")
            );

        [HttpPost]
        public async Task<ActionResult<Guid>> Long([FromBody] InputData data) =>
            (await SendToRabbit(BackendMessageType.Long, data))
            .Match(
                outputData => throw new Exception("Internal error: unexpected message result from SendToRabbit"),
                guid => guid
            );

        [HttpGet]
        public async Task<ActionResult> Result(Guid id)
        {
            // check in redis and get a result or null
            var s = id.ToString().ToLower();
            var db = _redisConnections.GetDatabase();
            var r = await db.StringGetAsync(s);

            if (r.IsNullOrEmpty)
            {
                return new ContentResult {StatusCode = 204};
            }

            return new ContentResult
            {
                Content = Newtonsoft.Json.JsonConvert.SerializeObject(_outputPreprocessor.PreprocessData(r)),
                ContentType = "application/json",
                StatusCode = 200
            };
        }


        ~PredictionController()
        {
            _channel.Close();
            _channel.Dispose(); // ?
            _conn.Close();
            _conn.Dispose(); // ?
            _redisConnections.Close();
            _redisConnections.Dispose(); // ?
        }
    }
}
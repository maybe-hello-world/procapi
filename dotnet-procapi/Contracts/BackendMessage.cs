using System;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace ProcAPI.Contracts
{
    [JsonConverter(typeof(StringEnumConverter))] 
    public enum BackendMessageType { Short, Long }

    [Serializable]
    public class BackendMessage
    {
        public BackendMessageType MessageType;
        public string Data;
        public Guid? Id;
    }
}
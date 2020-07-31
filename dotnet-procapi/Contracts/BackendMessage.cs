using System;
using System.Runtime.Serialization;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace ProcAPI.Contracts
{
    [JsonConverter(typeof(StringEnumConverter))]
    public enum BackendMessageType
    {
        [EnumMember(Value = "short")]
        Short,
        
        [EnumMember(Value = "long")]
        Long
    }

    [Serializable]
    public class BackendMessage
    {
        [JsonProperty(PropertyName = "message_type")]
        public BackendMessageType MessageType;
        
        [JsonProperty(PropertyName = "data")]
        public string Data;
        
        [JsonProperty(PropertyName = "id")]
        public Guid? Id;
    }
}
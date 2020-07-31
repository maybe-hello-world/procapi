using Newtonsoft.Json;

namespace ProcAPI.Contracts
{
    public class InputData
    {
        [JsonProperty(PropertyName = "img64")]
        public string Img64 { get; set; }
    }
}
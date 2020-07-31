using Newtonsoft.Json;

namespace ProcAPI.Contracts
{
    public class OutputData
    {
        [JsonProperty(PropertyName = "result_class")]
        public string ResultClass;
    }
}
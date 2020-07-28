using System.Collections.Generic;
using ProcAPI.Contracts;

namespace ProcAPI.Preprocessors
{
    public class OutputPreprocessor : IPreprocessor<string, OutputData>
    {
        private readonly Dictionary<int, string> _map; 

        public OutputPreprocessor()
        {
            _map = new Dictionary<int, string>
            {
                {0, "cat"},
                {1, "dog"}
            };
        }

        public OutputData PreprocessData(string data) => new OutputData
        {
            result_class = 
                int.TryParse(data, out var parsedInt) && 
                _map.TryGetValue(parsedInt, out var value)
                ? value
                : "unknown"
        };
    }
}
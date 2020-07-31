using System.Collections.Generic;
using ProcAPI.Contracts;

namespace ProcAPI.Preprocessors
{
    public class OutputPreprocessor : IPreprocessor<string, OutputData>
    {
        private readonly Dictionary<string, string> _map;

        public OutputPreprocessor()
        {
            _map = new Dictionary<string, string>
            {
                {"0", "cat"},
                {"1", "dog"}
            };
        }

        public OutputData PreprocessData(string data) => new OutputData
        {
            ResultClass =
                _map.TryGetValue(data, out var value)
                    ? value
                    : "unknown"
        };
    }
}
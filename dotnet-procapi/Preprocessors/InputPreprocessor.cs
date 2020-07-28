using System;
using System.IO;
using ProcAPI.Contracts;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Processing;

namespace ProcAPI.Preprocessors
{
    public class InputPreprocessor : IPreprocessor<InputData, string>
    {
        private const int Width = 256;
        private const int Height = 256;

        public string PreprocessData(InputData data)
        {
            // parse from base64
            var imgBytes = Convert.FromBase64String(data.img64);
            using var ms = new MemoryStream(imgBytes);
            using var img = Image.Load(ms);

            // resize
            img.Mutate(x =>
            {
                x.Resize(Width, Height);
                x.Grayscale();
            });

            // to base64
            using var oms = new MemoryStream();
            img.SaveAsJpeg(oms);
            var output64 = Convert.ToBase64String(oms.GetBuffer());

            return output64;
        }
    }
}
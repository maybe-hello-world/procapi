using System;
using Microsoft.AspNetCore.Diagnostics;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using SixLabors.ImageSharp;

namespace ProcAPI.Controllers
{
    [ApiExplorerSettings(IgnoreApi = true)]
    public class ErrorsController : ControllerBase
    {
        private readonly ILogger<ErrorsController> _logger;

        public ErrorsController(ILogger<ErrorsController> logger)
        {
            _logger = logger;
        }


        [Route("error")]
        public ActionResult Error()
        {
            var context = HttpContext.Features.Get<IExceptionHandlerFeature>();
            var exception = context?.Error;

            Response.StatusCode = exception switch
            {
                FormatException _ => 400,
                InvalidImageContentException _ => 400,
                _ => 500
            };

            _logger.LogError(exception?.Message);

            return Content(exception?.Message); // Your error model
        }
    }
}
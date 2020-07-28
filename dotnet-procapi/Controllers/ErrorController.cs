using System;
using Microsoft.AspNetCore.Diagnostics;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

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
            var code = 500; // Internal Server Error by default

            if (exception is FormatException) code = 400;

            Response.StatusCode = code;

            _logger.LogError(exception?.Message);

            return Content(exception?.Message); // Your error model
        }
    }
}
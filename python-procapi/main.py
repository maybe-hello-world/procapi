import binascii

import PIL
import uvicorn
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from Routers import prediction

app = FastAPI()
app.include_router(prediction.router, prefix="/prediction")


@app.exception_handler(binascii.Error)
@app.exception_handler(PIL.UnidentifiedImageError)
async def user_exceptions(request: Request, exc: Exception):
    return JSONResponse(
        status_code=400,
        content=str(exc)
    )


@app.exception_handler(binascii.Error)
async def default_exception_handler(request: Request, exc: Exception):
    return JSONResponse(
        status_code=500,
        content=str(exc)
    )


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=5001)

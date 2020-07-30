from pydantic import BaseModel


class InputData(BaseModel):
    img64: str

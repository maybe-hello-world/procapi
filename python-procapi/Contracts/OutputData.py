from pydantic import BaseModel


class OutputData(BaseModel):
    result_class: str

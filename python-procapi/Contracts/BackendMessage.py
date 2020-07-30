from enum import Enum
from dataclasses import dataclass
from typing import Optional
from uuid import UUID


class BackendMessageType(Enum):
    Short = "Short"
    Long = "Long"

    def __str__(self):
        return self.value


@dataclass(frozen=True)
class BackendMessage:
    MessageType: BackendMessageType
    Data: str
    Id: Optional[UUID]

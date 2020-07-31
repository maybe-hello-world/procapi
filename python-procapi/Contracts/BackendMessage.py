from enum import Enum
from dataclasses import dataclass
from typing import Optional
from uuid import UUID


class BackendMessageType(Enum):
    Short = "short"
    Long = "long"

    def __str__(self):
        return self.value


@dataclass(frozen=True)
class BackendMessage:
    message_type: BackendMessageType
    data: str
    id: Optional[UUID]

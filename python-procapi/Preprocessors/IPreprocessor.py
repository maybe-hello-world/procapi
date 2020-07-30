from abc import abstractmethod
from typing import Protocol, TypeVar

T = TypeVar('T')
TR = TypeVar('TR')


class IPreprocessor(Protocol):
    @abstractmethod
    def preprocess_data(self, data: T) -> TR:
        raise NotImplementedError

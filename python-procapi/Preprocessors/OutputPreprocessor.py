from Contracts.OutputData import OutputData
from Preprocessors.IPreprocessor import IPreprocessor


class OutputPreprocessor(IPreprocessor):
    __result_map = {
        "0": "cat",
        "1": "dog"
    }

    def preprocess_data(self, data: str) -> OutputData:
        return OutputData(result_class=self.__result_map.get(data, "unknown"))

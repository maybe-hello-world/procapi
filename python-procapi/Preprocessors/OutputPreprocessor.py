from Contracts.OutputData import OutputData
from Preprocessors.IPreprocessor import IPreprocessor
from Utils.parsers import try_parse_int


class OutputPreprocessor(IPreprocessor):
    __result_map = {
        0: "cat",
        1: "dog"
    }

    def preprocess_data(self, data: str) -> OutputData:
        return OutputData(
            result_class=self.__result_map.get(parsed_int, "unknown")
            if ((parsed_int := try_parse_int(data)) is not None)
            else "unknown"
        )

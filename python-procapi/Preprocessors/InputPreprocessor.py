from Contracts.InputData import InputData
from Preprocessors.IPreprocessor import IPreprocessor
from base64 import b64decode, b64encode
from PIL import Image
import io


class InputPreprocessor(IPreprocessor):
    __width = 256
    __height = 256

    def preprocess_data(self, data: InputData) -> str:
        buf = io.BytesIO(
            b64decode(data.img64.encode())
        )
        img = Image.open(buf)
        img = img.resize((self.__width, self.__height)).convert(mode='L')

        buf = io.BytesIO()
        img.save(buf, format="JPEG")
        return b64encode(buf.getvalue())



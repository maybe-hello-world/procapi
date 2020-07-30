from typing import Optional


def try_parse_int(x: str) -> Optional[int]:
    try:
        return int(x)
    except ValueError:
        return None

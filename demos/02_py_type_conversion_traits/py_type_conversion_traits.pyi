from typing import Callable
from datetime import date
from typing import NamedTuple

class Point(NamedTuple):
    x: float
    y: float

    def distance(self, other: Point) -> float: ...

class Person:
    first_name: str
    last_name: str

    def __init__(self, first_name: str, last_name: str) -> None: ...
    def full_name(self) -> str: ...

def add(a: float, b: float) -> float: ...
def map_float(
    py_transform_fn: Callable[[float], float], list: list[float]
) -> list[float]: ...
def extract_strings_from_dict(
    d: dict[str, str],
) -> tuple[list[str], list[str]]: ...
def get_weekdays(first_date: date, last_date: date) -> list[date]: ...

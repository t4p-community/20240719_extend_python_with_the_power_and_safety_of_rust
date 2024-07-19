# imported from the Rust extension
from .py_parallelism import (
    search_sequential,
    search_sequential_release_gil,
    search_parallel,
    search_parallel_release_gil,
    search_with_threads,
    search_with_threads_release_gil,
)

__all__ = [
    "search_py",  # defined at the bottom of this file
    "search_sequential",
    "search_sequential_release_gil",
    "search_parallel",
    "search_parallel_release_gil",
    "search_with_threads",
    "search_with_threads_release_gil",
]


def search_py(contents: str, needle: str) -> int:
    total = 0
    for line in contents.splitlines():
        for word in line.split(" "):
            if word == needle:
                total += 1
    return total

from concurrent.futures import ThreadPoolExecutor
from typing import Any

import pytest
import py_parallelism


@pytest.fixture(scope="session")
def contents() -> str:
    text = """
The Zen of Python, by Tim Peters

Beautiful is better than ugly.
Explicit is better than implicit.
Simple is better than complex.
Complex is better than complicated.
Flat is better than nested.
Sparse is better than dense.
Readability counts.
Special cases aren't special enough to break the rules.
Although practicality beats purity.
Errors should never pass silently.
Unless explicitly silenced.
In the face of ambiguity, refuse the temptation to guess.
There should be one-- and preferably only one --obvious way to do it.
Although that way may not be obvious at first unless you're Dutch.
Now is better than never.
Although never is often better than *right* now.
If the implementation is hard to explain, it's a bad idea.
If the implementation is easy to explain, it may be a good idea.
Namespaces are one honking great idea -- let's do more of those!
"""
    return text * 1000


def test_word_count_python_sequential(benchmark: Any, contents: str) -> None:
    count = benchmark(py_parallelism.search_py, contents, "is")
    assert count == 10000


def test_word_count_rust_sequential(benchmark: Any, contents: str) -> None:
    count = benchmark(py_parallelism.search_sequential, contents, "is")
    assert count == 10000


def test_word_count_rust_sequential_release_gil(
    benchmark: Any, contents: str
) -> None:
    count = benchmark(
        py_parallelism.search_sequential_release_gil, contents, "is"
    )
    assert count == 10000


def test_word_count_rust_parallel(benchmark: Any, contents: str) -> None:
    count = benchmark(py_parallelism.search_parallel, contents, "is")
    assert count == 10000


def test_word_count_rust_parallel_release_gil(
    benchmark: Any, contents: str
) -> None:
    count = benchmark(
        py_parallelism.search_parallel_release_gil, contents, "is"
    )
    assert count == 10000


def test_word_count_rust_parallel_threads(
    benchmark: Any, contents: str
) -> None:
    count = benchmark(py_parallelism.search_with_threads, contents, "is")
    assert count == 20000


def test_word_count_rust_parallel_threads_release_gil(
    benchmark: Any, contents: str
) -> None:
    count = benchmark(
        py_parallelism.search_with_threads_release_gil, contents, "is"
    )
    assert count == 20000


# variations of searches


# def run_python_sequential_four_times(contents: str, needle: str) -> None:
#     count = py_parallelism.search_py(contents, "is")
#     count += py_parallelism.search_py(contents, "is")
#     count += py_parallelism.search_py(contents, "is")
#     count += py_parallelism.search_py(contents, "is")
#     return count


# def test_word_count_python_sequential_four_times(
#     benchmark: Any, contents: str
# ) -> None:
#     count = benchmark(run_python_sequential_four_times, contents, "is")
#     assert count == 40000


# def run_python_sequential_four_times_threaded(
#     executor: ThreadPoolExecutor, contents: str, needle: str
# ) -> None:
#     future_1 = executor.submit(py_parallelism.search_py, contents, needle)
#     future_2 = executor.submit(py_parallelism.search_py, contents, needle)
#     future_3 = executor.submit(py_parallelism.search_py, contents, needle)
#     future_4 = executor.submit(py_parallelism.search_py, contents, needle)
#     result_1 = future_1.result()
#     result_2 = future_2.result()
#     result_3 = future_3.result()
#     result_4 = future_4.result()
#     return result_1 + result_2 + result_3 + result_4


# def test_word_count_python_sequential_four_times_threaded(
#     benchmark: Any, contents: str
# ) -> None:
#     executor = ThreadPoolExecutor(max_workers=4)
#     count = benchmark(
#         run_python_sequential_four_times_threaded, executor, contents, "is"
#     )
#     assert count == 40000


# def run_rust_sequential_four_times_keep_gil(
#     executor: ThreadPoolExecutor, contents: str, needle: str
# ) -> int:
#     future_1 = executor.submit(
#         py_parallelism.search_sequential, contents, needle
#     )
#     future_2 = executor.submit(
#         py_parallelism.search_sequential, contents, needle
#     )
#     future_3 = executor.submit(
#         py_parallelism.search_sequential, contents, needle
#     )
#     future_4 = executor.submit(
#         py_parallelism.search_sequential, contents, needle
#     )
#     result_1 = future_1.result()
#     result_2 = future_2.result()
#     result_3 = future_3.result()
#     result_4 = future_4.result()
#     return result_1 + result_2 + result_3 + result_4


# def test_word_count_rust_sequential_four_times_with_threads_keep_gil(
#     benchmark: Any, contents: str
# ) -> None:
#     executor = ThreadPoolExecutor(max_workers=4)
#     count = benchmark(
#         run_rust_sequential_four_times_keep_gil, executor, contents, "is"
#     )
#     assert count == 40000


# def run_rust_sequential_four_times_release_gil(
#     executor: ThreadPoolExecutor, contents: str, needle: str
# ) -> int:
#     future_1 = executor.submit(
#         py_parallelism.search_sequential_release_gil, contents, needle
#     )
#     future_2 = executor.submit(
#         py_parallelism.search_sequential_release_gil, contents, needle
#     )
#     future_3 = executor.submit(
#         py_parallelism.search_sequential_release_gil, contents, needle
#     )
#     future_4 = executor.submit(
#         py_parallelism.search_sequential_release_gil, contents, needle
#     )
#     result_1 = future_1.result()
#     result_2 = future_2.result()
#     result_3 = future_3.result()
#     result_4 = future_4.result()
#     return result_1 + result_2 + result_3 + result_4


# def test_word_count_rust_sequential_four_times_release_gil(
#     benchmark: Any, contents: str
# ) -> None:
#     executor = ThreadPoolExecutor(max_workers=4)
#     count = benchmark(
#         run_rust_sequential_four_times_release_gil, executor, contents, "is"
#     )
#     assert count == 40000

use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

// count the occurrences of needle in line, case insensitive
fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if word == needle {
            total += 1;
        }
    }
    total
}

/// searches for a word, sequentially by line
#[pyfunction]
fn search_sequential(contents: &str, needle: &str) -> usize {
    contents.lines().map(|line| count_line(line, needle)).sum()
}

#[pyfunction]
fn search_sequential_release_gil(py: Python<'_>, contents: &str, needle: &str) -> usize {
    // releases the Python GIL
    py.allow_threads(|| search_sequential(contents, needle))
}

/// searches for the word, parallelized by line
#[pyfunction]
fn search_parallel(contents: &str, needle: &str) -> usize {
    contents
        // parallelized by rayon
        .par_lines()
        .map(|line| count_line(line, needle))
        .sum()
}

#[pyfunction]
fn search_parallel_release_gil(py: Python<'_>, contents: &str, needle: &str) -> usize {
    // releases the Python GIL
    py.allow_threads(|| search_parallel(contents, needle))
}

// performs the search one time for each cpu * core
#[pyfunction]
fn search_with_threads(contents: &str, needle: &str) -> usize {
    let count = Arc::new(Mutex::new(0));
    let contents = Arc::new(contents.to_string());
    let needle = Arc::new(needle.to_string());
    let mut handles = vec![];
    // 0..x, x should be set to the number of cpu cores
    // codespace devcontainer has 2 cores by default
    for _ in 0..2 {
        let count = Arc::clone(&count);
        let contents = Arc::clone(&contents);
        let needle = Arc::clone(&needle);
        let handle = thread::spawn(move || {
            let sum: usize = contents
                .par_lines()
                .map(|line| count_line(line, &needle))
                .sum();
            let mut c = count.lock().unwrap();
            *c += sum;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return *count.lock().unwrap();
}

#[pyfunction]
fn search_with_threads_release_gil(py: Python<'_>, contents: &str, needle: &str) -> usize {
    // releases the Python GIL
    py.allow_threads(|| search_with_threads(contents, needle))
}

#[pymodule]
fn py_parallelism(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(search_sequential, m)?)?;
    m.add_function(wrap_pyfunction!(search_sequential_release_gil, m)?)?;
    m.add_function(wrap_pyfunction!(search_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(search_parallel_release_gil, m)?)?;
    m.add_function(wrap_pyfunction!(search_with_threads, m)?)?;
    m.add_function(wrap_pyfunction!(search_with_threads_release_gil, m)?)?;
    Ok(())
}

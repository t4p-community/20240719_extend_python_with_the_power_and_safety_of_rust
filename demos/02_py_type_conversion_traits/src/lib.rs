use std::collections::HashMap;

use chrono::prelude::*;
use chrono::NaiveDate;
use pyo3::prelude::*;
use pyo3::types::PyAny;

// dd two f64 values and return an f64
#[pyfunction]
fn add(a: f64, b: f64) -> PyResult<f64> {
    Ok(a + b)
}

#[pyfunction]
fn map_float(py_transform_func: &PyAny, list: Vec<f64>) -> PyResult<Vec<f64>> {
    let mut new_list = vec![];

    for item in list {
        let new_item = py_transform_func.call1((item,))?;
        let result: f64 = new_item.extract()?;
        new_list.push(result);
    }

    Ok(new_list)
}

#[pyfunction]
fn extract_strings_from_dict(
    dict: HashMap<String, String>,
) -> PyResult<(Vec<String>, Vec<String>)> {
    let mut keys: Vec<String> = vec![];
    let mut values: Vec<String> = vec![];

    for (key, value) in dict {
        keys.push(key);
        values.push(value);
    }

    Ok((keys, values))
}

#[pyfunction]
fn get_weekdays(first_date: NaiveDate, last_date: NaiveDate) -> PyResult<Vec<NaiveDate>> {
    let mut weekdays: Vec<NaiveDate> = vec![];
    let mut date = first_date;

    while date < last_date {
        if date.weekday() != Weekday::Sat && date.weekday() != Weekday::Sun {
            weekdays.push(date);
        }
        match date.succ_opt() {
            Some(d) => date = d,
            None => break,
        }
    }

    Ok(weekdays)
}

#[pyclass]
struct Point(f64, f64);

#[pymethods]
impl Point {
    #[new]
    fn py_new(x: f64, y: f64) -> PyResult<Self> {
        Ok(Point(x, y))
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

#[pyclass]
struct Person {
    first_name: String,
    last_name: String,
}

#[pymethods]
impl Person {
    #[new]
    fn py_new(first_name: &str, last_name: &str) -> PyResult<Self> {
        Ok(Person {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
        })
    }

    fn full_name(&self) -> PyResult<String> {
        Ok(format!("{}, {}", self.last_name, self.first_name))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_type_conversion_traits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(map_float, m)?)?;
    m.add_function(wrap_pyfunction!(extract_strings_from_dict, m)?)?;
    m.add_function(wrap_pyfunction!(get_weekdays, m)?)?;
    m.add_class::<Point>()?;
    m.add_class::<Person>()?;
    Ok(())
}

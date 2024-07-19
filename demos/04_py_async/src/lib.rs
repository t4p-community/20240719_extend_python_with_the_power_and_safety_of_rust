use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::{prelude::*, wrap_pyfunction};
use regex::Regex;
use reqwest::Client;

const YAHOO_FINANCE_URL: &str = "https://finance.yahoo.com/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
const LAST_PRICE_PATTERN: &str = r#"data-field="regularMarketPrice" data-trend="none" data-pricehint="2" data-value="(?<price>[0-9.]+)""#;

#[pyfunction]
fn rust_sleep(py: Python<'_>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pyfunction]
fn echo_param(py: Python<'_>, param: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(Python::with_gil(|_| param))
    })
}

#[pyfunction]
fn print_list(py: Python<'_>, items: Vec<String>) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        for item in items {
            println!("{}", item);
        }
        Ok(Python::with_gil(|py| py.None()))
    })
}

fn extract_price(last_price_re: &Regex, html: &str) -> Option<f64> {
    let caps = last_price_re.captures(html)?;
    let price = caps.name("price")?;
    let Ok(price) = price.as_str().parse::<f64>() else {
        return None;
    };
    Some(price)
}

create_exception!(py_type_conversion_traits, StockPriceError, PyException);

#[pyfunction]
fn get_stock_price(py: Python<'_>, stock_symbol: String) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        return Err(PyErr::new::<StockPriceError, _>(format!("fake error")));

        let last_price_re = Regex::new(LAST_PRICE_PATTERN)
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))?;

        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))?;

        let url = format!("{}/quote/{}/", YAHOO_FINANCE_URL, stock_symbol);
        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))?;

        let body = resp
            .text()
            .await
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{}", e)))?;

        if let Some(price) = extract_price(&last_price_re, &body) {
            return Ok(Python::with_gil(|_| price.to_string()));
        }

        Err(PyErr::new::<PyValueError, _>(
            "Unable to extract price from response.",
        ))
    })
}

#[pymodule]
fn py_async(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    m.add_function(wrap_pyfunction!(echo_param, m)?)?;
    m.add_function(wrap_pyfunction!(print_list, m)?)?;
    m.add_function(wrap_pyfunction!(get_stock_price, m)?)?;
    m.add("StockPriceError", py.get_type::<StockPriceError>())?;
    Ok(())
}

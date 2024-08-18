mod backend;

use pyo3::prelude::*;
use pyo3::types::PyString;

/// Returns the length of the longest shared subsequence
#[pyfunction]
fn longest_subsequence_len(a: &PyString, b: &PyString) -> PyResult<u64> {
    let a: &str = a.to_str().unwrap();
    let b: &str = b.to_str().unwrap();

    let output = backend::longest_subsequence_len(a, b);

    PyResult::Ok(output)
}

///
#[pyfunction]
fn longest_subsequence<'a>(
    py: Python<'a>,
    a: &'a PyString,
    b: &'a PyString,
) -> PyResult<&'a PyString> {
    let a: &str = a.to_str().unwrap();
    let b: &str = b.to_str().unwrap();

    let output = backend::longest_subsequence_make(a, b);

    PyResult::Ok(PyString::new(py, &output))
}

/// A Python module implemented in Rust.
#[pymodule]
fn subsequence_finder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(longest_subsequence_len, m)?)?;
    m.add_function(wrap_pyfunction!(longest_subsequence, m)?)?;
    Ok(())
}

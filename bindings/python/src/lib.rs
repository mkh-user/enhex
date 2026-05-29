use pyo3::prelude::*;
use enhex_core::compile as core_compile;

/// Compile an EnhEx pattern string to a Regex string.
#[pyfunction]
fn compile(_py: Python<'_>, pattern: &str) -> PyResult<String> {
    core_compile(pattern)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// EnhEx - Enhanced Expression.
/// A readable language for writing regular expressions.
#[pymodule]
fn _enhex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile, m)?)?;
    Ok(())
}

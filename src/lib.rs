use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// We rename the Rust library to `ansi_to_html_lib` to avoid name clash
use ansi_to_html_lib::convert as _convert;

/// Render an ANSI string to HTML.
#[pyfunction(signature=(text))]
fn convert(text: &str) -> PyResult<String> {
    match _convert(text) {
        Ok(html) => Ok(html),
        Err(err) => Err(PyValueError::new_err(format!(
            "ANSI conversion error: {}",
            err
        ))),
    }
}

#[pymodule]
fn ansi_to_html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Expose the wrapped function
    m.add_function(wrap_pyfunction!(convert, m)?)?;

    Ok(())
}

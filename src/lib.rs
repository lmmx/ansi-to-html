use pyo3::prelude::*;

// We rename the Rust library to `ansi_to_html_lib` to avoid name clash
use ansi_to_html_lib::{convert as _convert};

/// Render an ANSI string to HTML.
#[pyfunction(signature=(text))]
fn convert(
    text: &str,
) -> PyResult<String> {
    let html = _convert(text);
    Ok(html)
}

#[pymodule]
fn ansi_to_html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Expose the wrapped function
    m.add_function(wrap_pyfunction!(convert, m)?)?;

    Ok(())
}

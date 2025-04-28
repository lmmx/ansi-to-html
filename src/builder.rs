use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// Import the Converter from the rust library
use ansi_to_html_lib::Converter as RsConverter;

/// Python wrapper for the ansi_to_html::Converter
///
/// Provides a builder pattern interface for configuring ANSI to HTML conversion.
#[pyclass]
pub struct Converter {
    inner: RsConverter,
}

#[pymethods]
impl Converter {
    /// Create a new Converter with default settings
    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> Self {
        Self {
            inner: RsConverter::new(),
        }
    }

    /// Skip escaping HTML special characters
    ///
    /// Args:
    ///     skip: Whether to skip escaping HTML special characters
    ///
    /// Returns:
    ///     Self: The converter instance with the updated setting
    pub fn skip_escape(&self, skip: bool) -> Self {
        Self {
            inner: self.inner.clone().skip_escape(skip),
        }
    }

    /// Skip optimizing the HTML output
    ///
    /// Args:
    ///     skip: Whether to skip optimizing the HTML output
    ///
    /// Returns:
    ///     Self: The converter instance with the updated setting
    pub fn skip_optimize(&self, skip: bool) -> Self {
        Self {
            inner: self.inner.clone().skip_optimize(skip),
        }
    }

    /// Set a custom prefix for CSS variables used for 4-bit colors
    ///
    /// Args:
    ///     prefix: The prefix to use for CSS variables, or None to use default
    ///
    /// Returns:
    ///     Self: The converter instance with the updated setting
    #[pyo3(signature = (prefix=None))]
    pub fn four_bit_var_prefix(&self, prefix: Option<String>) -> Self {
        Self {
            inner: self.inner.clone().four_bit_var_prefix(prefix),
        }
    }

    /// Convert ANSI text to HTML using the configured settings
    ///
    /// Args:
    ///     text: The ANSI text to convert
    ///
    /// Returns:
    ///     str: The converted HTML
    ///
    /// Raises:
    ///     ValueError: If the conversion fails
    pub fn convert(&self, text: &str) -> PyResult<String> {
        match self.inner.convert(text) {
            Ok(html) => Ok(html),
            Err(err) => Err(PyValueError::new_err(format!(
                "ANSI conversion error: {}",
                err
            ))),
        }
    }
}

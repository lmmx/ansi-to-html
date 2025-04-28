use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// Import the Converter from the Rust library
use ansi_to_html_lib::Converter as RustConverter;

/// Python wrapper for ansi_to_html::Converter that accepts initialization flags
/// instead of using the builder pattern
#[pyclass]
pub struct Converter {
    inner: RustConverter,
}

#[pymethods]
impl Converter {
    /// Create a new Converter with optional configuration flags
    ///
    /// Args:
    ///     skip_escape: Whether to skip escaping HTML special characters
    ///     skip_optimize: Whether to skip optimizing HTML output
    ///     four_bit_var_prefix: Custom prefix for CSS variables used for 4-bit colors
    #[pyo3(signature = (skip_escape = false, skip_optimize = false, four_bit_var_prefix = None),
      text_signature = "(self, skip_escape=False, skip_optimize=False, four_bit_var_prefix=None)")]
    #[new]
    pub fn new(
        skip_escape: bool,
        skip_optimize: bool,
        four_bit_var_prefix: Option<String>,
    ) -> Self {
        // Create base converter
        let mut converter = RustConverter::new();

        // Apply all settings directly using the builder pattern in Rust
        converter = converter.skip_escape(skip_escape);
        converter = converter.skip_optimize(skip_optimize);

        // Only set the prefix if provided
        if let Some(prefix) = four_bit_var_prefix {
            converter = converter.four_bit_var_prefix(Some(prefix));
        }

        Self { inner: converter }
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

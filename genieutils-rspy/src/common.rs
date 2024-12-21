use genieutils::common::DebugString;
use pyo3::prelude::*;

#[pyclass(module = "genieutils_rspy", name = "DebugString")]
pub struct PyDebugString {
    int_str: String,
}

impl From<DebugString> for PyDebugString {
    fn from(value: DebugString) -> Self {
        Self {
            int_str: value.int_str,
        }
    }
}

impl From<String> for PyDebugString {
    fn from(value: String) -> Self {
        Self {
            int_str: value,
        }
    }
}

impl From<PyDebugString> for DebugString {
    fn from(value: PyDebugString) -> Self {
        Self {
            int_str: value.int_str,
        }
    }
}

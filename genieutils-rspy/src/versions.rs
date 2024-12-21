use genieutils::versions::Version;
use pyo3::prelude::*;

#[pyclass(name = "Version", module = "genieutils_rspy")]
pub struct PyVersion(Version);

impl From<Version> for PyVersion {
    fn from(value: Version) -> Self {
        Self(value)
    }
}

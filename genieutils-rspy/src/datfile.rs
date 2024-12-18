use genieutils::datfile::DatFile;
use pyo3::prelude::*;
use pyo3::pyclass;

#[pyclass(name = "DatFile", module = "genieutils_rspy")]
pub struct PyDatFile {
    #[pyo3(get)]
    pub datfile: Py<DatFile>,
}

impl From<PyDatFile> for Py<DatFile> {
    fn from(datfile: PyDatFile) -> Py<DatFile> {
        datfile.datfile
    }
}

impl From<Py<DatFile>> for PyDatFile {
    fn from(datfile: Py<DatFile>) -> PyDatFile {
        PyDatFile { datfile }
    }
}

use pyo3::exceptions::PyValueError;

#[pymethods]
impl PyDatFile {
    #[staticmethod]
    #[pyo3(name = "parse_compressed")]
    fn py_parse_compressed(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        let datfile = DatFile::parse_compressed(data)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        let pydatfile = Py::new(py, datfile)?;
        Ok(pydatfile.into())
    }

    #[staticmethod]
    #[pyo3(name = "parse")]
    fn py_parse(py: Python<'_>, data: Vec<u8>) -> PyResult<Self> {
        let datfile =
            DatFile::parse(&data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        let pydatfile = Py::new(py, datfile)?;
        Ok(pydatfile.into())
    }

    #[staticmethod]
    #[pyo3(name = "decompress")]
    fn py_decompress(data: &[u8]) -> PyResult<Vec<u8>> {
        let data =
            DatFile::decompress(data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    #[pyo3(name = "serialize")]
    fn py_serialize(&self, py: Python<'_>) -> PyResult<Vec<u8>> {
        let data = self
            .datfile
            .bind(py)
            .borrow()
            .serialize()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    #[pyo3(name = "pack")]
    fn py_pack(&self, py: Python<'_>) -> PyResult<Vec<u8>> {
        let data = self
            .datfile
            .bind(py)
            .borrow()
            .pack()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }
}

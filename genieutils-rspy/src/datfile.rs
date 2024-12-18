use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use genieutils::datfile::DatFile;

#[pyclass(name = "DatFile", module = "genieutils_rspy", subclass)]
pub struct PyDatFile {
    pub(crate) datfile: DatFile,
}

impl From<PyDatFile> for DatFile {
    fn from(datfile: PyDatFile) -> DatFile {
        datfile.datfile
    }
}

impl From<DatFile> for PyDatFile {
    fn from(datfile: DatFile) -> PyDatFile {
        PyDatFile { datfile }
    }
}

#[pymethods]
impl PyDatFile {
    #[staticmethod]
    fn parse_compressed(data: &[u8]) -> PyResult<Self> {
        let datfile = DatFile::parse_compressed(data)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(datfile.into())
    }

    #[staticmethod]
    fn parse(data: Vec<u8>) -> PyResult<Self> {
        let datfile =
            DatFile::parse(&data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(datfile.into())
    }

    #[staticmethod]
    fn decompress(data: &[u8]) -> PyResult<Vec<u8>> {
        let data =
            DatFile::decompress(data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    fn serialize(&self) -> PyResult<Vec<u8>> {
        let data = self
            .datfile
            .serialize()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    fn pack(&self) -> PyResult<Vec<u8>> {
        let data = self
            .datfile
            .pack()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    #[getter]
    fn version(&self) -> PyResult<u8> {
        Ok(self.datfile.version as u8)
    }
}

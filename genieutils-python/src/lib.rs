use pyo3::prelude::*;

use genieutils::datfile::python::PyDatFile;

#[pymodule]
fn genieutils_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDatFile>()?;
    Ok(())
}

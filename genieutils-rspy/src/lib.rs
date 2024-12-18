use pyo3::prelude::*;

//mod datfile;
//mod playercolour;

use genieutils::datfile::DatFile;

/// A Python module implemented in Rust.
#[pymodule]
fn genieutils_rspy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DatFile>()?;
    Ok(())
}

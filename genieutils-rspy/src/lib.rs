use pyo3::prelude::*;

mod datfile;

/// A Python module implemented in Rust.
#[pymodule]
fn genieutils_rspy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<datfile::PyDatFile>()?;
    Ok(())
}

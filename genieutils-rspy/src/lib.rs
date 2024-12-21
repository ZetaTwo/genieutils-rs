use pyo3::prelude::*;

//mod common;
mod datfile;
//mod playercolour;
//mod playercolour;
//mod sound;
//mod terrainrestrictions;
//mod versions;

//use genieutils::datfile::DatFile;

/// A Python module implemented in Rust.
#[pymodule]
fn genieutils_rspy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<datfile::PyDatFile>()?;
    Ok(())
}

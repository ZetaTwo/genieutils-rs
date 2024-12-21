use genieutils::datfile::DatFile;
use pyo3::prelude::*;
/*
use pyo3::pyclass;
use pyo3::types::PyList;

use crate::playercolour::PyPlayerColour;
use crate::sound::PySound;
use crate::terrainrestrictions::PyTerrainRestriction;
use crate::versions::PyVersion;

#[pyclass(name = "DatFile", module = "genieutils_rspy")]
pub struct PyDatFile {
    #[pyo3(get, set)]
    pub version: Py<PyVersion>,
    #[pyo3(get, set)]
    pub float_ptr_terrain_tables: Py<PyList>,
    #[pyo3(get, set)]
    pub terrain_pass_graphic_pointers: Py<PyList>,
    #[pyo3(get, set)]
    pub terrain_restrictions: Py<PyList>,
    #[pyo3(get, set)]
    pub player_colours: Py<PyList>,
    #[pyo3(get, set)]
    pub sounds: Py<PyList>,
    #[pyo3(get, set)]
    graphic_pointers: Py<PyList>,
    //#[pyo3(get, set)]
    //pub graphics: Vec<Option<Py<PyGraphic>>>,
    //#[pyo3(get, set)]
    //pub terrain_block: Py<PyTerrainBlock>,
    //#[pyo3(get, set)]
    //pub random_maps: Py<PyRandomMaps>,
    //#[pyo3(get, set)]
    //pub effects: Vec<Py<PyEffect>>,
    //#[pyo3(get, set)]
    //pub unit_headers: Vec<Py<PyUnitHeaders>>,
    //#[pyo3(get, set)]
    //pub civs: Vec<Py<PyCiv>>,
    //#[pyo3(get, set)]
    //pub techs: Vec<Py<PyTech>>,
    pub time_slice: u32,
    pub unit_kill_rate: u32,
    pub unit_kill_total: u32,
    pub unit_hit_point_rate: u32,
    pub unit_hit_point_total: u32,
    pub razing_kill_rate: u32,
    pub razing_kill_total: u32,
    //pub tech_tree: Py<PyTechTree>,
}

impl TryFrom<DatFile> for PyDatFile {
    type Error = PyErr;

    fn try_from(value: DatFile) -> PyResult<Self> {
        Python::with_gil(|py| -> PyResult<_> {
            let version: PyVersion = value.version.into();

            let terrain_restrictions: Vec<PyTerrainRestriction> = value
                .terrain_restrictions
                .into_iter()
                .map(PyTerrainRestriction::try_from)
                .collect::<PyResult<Vec<PyTerrainRestriction>>>()?;

            let player_colours: Vec<PyPlayerColour> = value
                .player_colours
                .into_iter()
                .map(PyPlayerColour::from)
                .collect();

            let sounds: Vec<PySound> = value
                .sounds
                .into_iter()
                .map(PySound::try_from)
                .collect::<PyResult<Vec<PySound>>>()?;

            let datfile = Self {
                version: Py::new(py, version)?,
                float_ptr_terrain_tables: PyList::new(py, value.float_ptr_terrain_tables)?.unbind(),
                terrain_pass_graphic_pointers: PyList::new(
                    py,
                    value.terrain_pass_graphic_pointers,
                )?
                .unbind(),
                terrain_restrictions: PyList::new(py, terrain_restrictions)?.unbind(),
                player_colours: PyList::new(py, player_colours)?.unbind(),
                sounds: PyList::new(py, sounds)?.unbind(),
                graphic_pointers: PyList::new(py, value.graphic_pointers)?.unbind(),
                //graphics: value.graphics,
                //terrain_block: value.terrain_block,
                //random_maps: value.random_maps,
                //effects: value.effects,
                //unit_headers: value.unit_headers,
                //civs: value.civs,
                //techs: value.techs,
                time_slice: value.time_slice,
                unit_kill_rate: value.unit_kill_rate,
                unit_kill_total: value.unit_kill_total,
                unit_hit_point_rate: value.unit_hit_point_rate,
                unit_hit_point_total: value.unit_hit_point_total,
                razing_kill_rate: value.razing_kill_rate,
                razing_kill_total: value.razing_kill_total,
                //tech_tree: value.tech_tree,
            };
            Ok(datfile)
        })
    }
}
*/
use pyo3::exceptions::PyValueError;

#[pyclass(module = "genieutils_rspy", name = "DatFile")]
pub struct PyDatFile {}

use pyo3::types::PyDict;

#[pymethods]
impl PyDatFile {
    #[staticmethod]
    #[pyo3(name = "parse_compressed")]
    fn py_parse_compressed(py: Python<'_>, data: &[u8]) -> PyResult<Py<PyDict>> {
        let datfile = DatFile::parse_compressed(data)
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        let res = datfile.into_pyobject(py)?;
        Ok(res.unbind())
    }

    #[staticmethod]
    #[pyo3(name = "parse")]
    fn py_parse(py: Python<'_>, data: Vec<u8>) -> PyResult<Py<PyDict>> {
        let datfile =
            DatFile::parse(&data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        let res = datfile.into_pyobject(py)?;
        Ok(res.unbind())
    }

    #[staticmethod]
    #[pyo3(name = "decompress")]
    fn py_decompress(data: &[u8]) -> PyResult<Vec<u8>> {
        let data =
            DatFile::decompress(data).map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    /*  #[pyo3(name = "serialize")]
    fn py_serialize(&self, py: Python<'_>) -> PyResult<Vec<u8>> {
        let data = self.borrow()

            .serialize()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }

    #[pyo3(name = "pack")]
    fn py_pack(&self, py: Python<'_>) -> PyResult<Vec<u8>> {
        let data = self
            .bind(py)
            .borrow()
            .pack()
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        Ok(data)
    }*/
}

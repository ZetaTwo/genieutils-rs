use pyo3::prelude::*;
use pyo3::types::PyList;

use genieutils::terrainrestrictions::{TerrainPassGraphic, TerrainRestriction};

#[pyclass(name = "TerrainPassGraphic", module = "genieutils_rspy")]
pub struct PyTerrainPassGraphic {
    #[pyo3(get, set)]
    exit_tile_sprite_id: i32,
    #[pyo3(get, set)]
    enter_tile_sprite_id: i32,
    #[pyo3(get, set)]
    walk_tile_sprite_id: i32,
    #[pyo3(get, set)]
    walk_sprite_rate: i32,
}

impl From<TerrainPassGraphic> for PyTerrainPassGraphic {
    fn from(value: TerrainPassGraphic) -> Self {
        Self {
            exit_tile_sprite_id: value.exit_tile_sprite_id,
            enter_tile_sprite_id: value.enter_tile_sprite_id,
            walk_tile_sprite_id: value.walk_tile_sprite_id,
            walk_sprite_rate: value.walk_sprite_rate,
        }
    }
}

#[pyclass(name = "TerrainRestriction", module = "genieutils_rspy")]
pub struct PyTerrainRestriction {
    #[pyo3(get, set)]
    pub passable_buildable_dmg_multiplier: Py<PyList>,
    #[pyo3(get, set)]
    pub terrain_pass_graphics: Py<PyList>,
}

impl TryFrom<TerrainRestriction> for PyTerrainRestriction {
    type Error = PyErr;

    fn try_from(value: TerrainRestriction) -> PyResult<Self> {
        let terrain_pass_graphics: Vec<PyTerrainPassGraphic> = value
            .terrain_pass_graphics
            .into_iter()
            .map(PyTerrainPassGraphic::from)
            .collect();

        Python::with_gil(|py| -> PyResult<_> {
            let result = Self {
                passable_buildable_dmg_multiplier: PyList::new(
                    py,
                    value.passable_buildable_dmg_multiplier,
                )?
                .unbind(),
                terrain_pass_graphics: PyList::new(py, terrain_pass_graphics)?.unbind(),
            };
            Ok(result)
        })
    }
}

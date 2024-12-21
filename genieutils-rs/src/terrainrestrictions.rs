use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct TerrainPassGraphic {
    pub exit_tile_sprite_id: i32,
    pub enter_tile_sprite_id: i32,
    pub walk_tile_sprite_id: i32,
    pub walk_sprite_rate: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
#[br(import(terrain_count: usize))]
#[bw(assert(passable_buildable_dmg_multiplier.len() == terrain_pass_graphics.len(), "terrain restriciton lists lengths unmatched: {} != {}", passable_buildable_dmg_multiplier.len(), terrain_pass_graphics.len()))]
pub struct TerrainRestriction {
    #[br(count = terrain_count)]
    pub passable_buildable_dmg_multiplier: Vec<f32>,
    #[br(count = terrain_count)]
    pub terrain_pass_graphics: Vec<TerrainPassGraphic>,
}

#[cfg(feature = "pyo3")]
pub mod python {
    use super::TerrainPassGraphic;
    use super::TerrainRestriction;

    use pyo3::prelude::*;
    use pyo3::types::PyList;

    #[pyclass(name = "TerrainPassGraphic", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTerrainPassGraphic {
        pub exit_tile_sprite_id: i32,
        pub enter_tile_sprite_id: i32,
        pub walk_tile_sprite_id: i32,
        pub walk_sprite_rate: i32,
    }

    impl<'py> IntoPyObject<'py> for TerrainPassGraphic {
        type Target = PyTerrainPassGraphic;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTerrainPassGraphic {
                exit_tile_sprite_id: self.exit_tile_sprite_id,
                enter_tile_sprite_id: self.enter_tile_sprite_id,
                walk_tile_sprite_id: self.walk_tile_sprite_id,
                walk_sprite_rate: self.walk_sprite_rate,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "TerrainRestriction", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTerrainRestriction {
        pub passable_buildable_dmg_multiplier: Py<PyList>,
        pub terrain_pass_graphics: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for TerrainRestriction {
        type Target = PyTerrainRestriction;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTerrainRestriction {
                passable_buildable_dmg_multiplier: self
                    .passable_buildable_dmg_multiplier
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                terrain_pass_graphics: self
                    .terrain_pass_graphics
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

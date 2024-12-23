use binrw::binrw;
use binrw::helpers::args_iter_with;
use binrw::BinRead;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::common::DebugString;
use crate::unit::Unit;
use crate::versions::Version;

#[binrw]
#[brw(import(version: Version))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
/// A structure representing a Genie engine civilization
pub struct Civ {
    pub player_type: u8,
    pub name: DebugString,

    #[br(temp)]
    #[bw(try_calc = resources.len().try_into())]
    resources_size: i16,

    pub tech_tree_id: i16,
    pub team_bonus_id: i16,
    #[br(count = resources_size)]
    pub resources: Vec<f32>,
    pub icon_set: u8,

    #[br(temp)]
    #[bw(try_calc = unit_pointers.len().try_into())]
    pub units_size: i16,

    #[br(count = units_size)]
    pub unit_pointers: Vec<i32>,

    #[br(parse_with = args_iter_with(&unit_pointers,
    |reader, endian, &pointer| {
        if pointer == 0 {
            Ok(None)
        } else {
            <Unit as BinRead>::read_options(reader, endian, (version,)).map(Some)
        }
    }
    ))]
    #[bw(args(version,))]
    pub units: Vec<Option<Unit>>,
}

#[cfg(feature = "pyo3")]
mod python {
    use super::Civ;

    use pyo3::prelude::*;
    use pyo3::types::PyList;
    use pyo3::types::PyString;
    #[pyclass(name = "Civ", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyCiv {
        player_type: u8,
        name: Py<PyString>,
        tech_tree_id: i16,
        team_bonus_id: i16,
        resources: Py<PyList>,
        icon_set: u8,
        unit_pointers: Py<PyList>,
        units: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for Civ {
        type Target = PyCiv;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyCiv {
                player_type: self.player_type,
                name: self.name.into_pyobject(py)?.unbind(),
                tech_tree_id: self.tech_tree_id,
                team_bonus_id: self.team_bonus_id,
                resources: self.resources.into_pyobject(py)?.downcast_into()?.unbind(),
                icon_set: self.icon_set,
                unit_pointers: self
                    .unit_pointers
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                units: self.units.into_pyobject(py)?.downcast_into()?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

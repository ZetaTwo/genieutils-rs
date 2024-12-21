use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct PlayerColour {
    pub id: i32,
    pub player_color_base: i32,
    pub unit_outline_color: i32,
    pub unit_selection_color_1: i32,
    pub unit_selection_color_2: i32,
    pub minimap_color: i32,
    pub minimap_color_2: i32,
    pub minimap_color_3: i32,
    pub statistics_text: i32,
}

#[cfg(feature = "pyo3")]
mod python {
    use super::PlayerColour;
    use pyo3::prelude::*;

    #[pyclass(name = "PlayerColour", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyPlayerColour {
        pub id: i32,
        pub player_color_base: i32,
        pub unit_outline_color: i32,
        pub unit_selection_color_1: i32,
        pub unit_selection_color_2: i32,
        pub minimap_color: i32,
        pub minimap_color_2: i32,
        pub minimap_color_3: i32,
        pub statistics_text: i32,
    }

    impl<'py> IntoPyObject<'py> for PlayerColour {
        type Target = PyPlayerColour;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyPlayerColour {
                id: self.id, 
                player_color_base: self.player_color_base, 
                unit_outline_color: self.unit_outline_color, 
                unit_selection_color_1: self.unit_selection_color_1, 
                unit_selection_color_2: self.unit_selection_color_2, 
                minimap_color: self.minimap_color, 
                minimap_color_2: self.minimap_color_2, 
                minimap_color_3: self.minimap_color_3, 
                statistics_text: self.statistics_text, 
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

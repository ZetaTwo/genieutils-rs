use pyo3::prelude::*;

use genieutils::playercolour::PlayerColour;

#[pyclass(name = "PlayerColour", module = "genieutils_rspy")]
pub struct PyPlayerColour {
    #[pyo3(get, set)]
    pub id: i32,
    #[pyo3(get, set)]
    pub player_color_base: i32,
    #[pyo3(get, set)]
    pub unit_outline_color: i32,
    #[pyo3(get, set)]
    pub unit_selection_color_1: i32,
    #[pyo3(get, set)]
    pub unit_selection_color_2: i32,
    #[pyo3(get, set)]
    pub minimap_color: i32,
    #[pyo3(get, set)]
    pub minimap_color_2: i32,
    #[pyo3(get, set)]
    pub minimap_color_3: i32,
    #[pyo3(get, set)]
    pub statistics_text: i32,
}

impl From<PlayerColour> for PyPlayerColour {
    fn from(value: PlayerColour) -> Self {
        Self {
            id: value.id,
            player_color_base: value.player_color_base,
            unit_outline_color: value.unit_outline_color,
            unit_selection_color_1: value.unit_selection_color_1,
            unit_selection_color_2: value.unit_selection_color_2,
            minimap_color: value.minimap_color,
            minimap_color_2: value.minimap_color_2,
            minimap_color_3: value.minimap_color_3,
            statistics_text: value.statistics_text,
        }
    }
}

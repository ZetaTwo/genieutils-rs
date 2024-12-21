use pyo3::prelude::*;

use genieutils::sound::{Sound, SoundItem};
use pyo3::types::PyList;

use crate::common::PyDebugString;

#[pyclass(name = "SoundItem", module = "genieutils_rspy")]
struct PySoundItem {
    #[pyo3(get, set)]
    pub filename: String,
    #[pyo3(get, set)]
    pub resource_id: i32,
    #[pyo3(get, set)]
    pub probability: i16,
    #[pyo3(get, set)]
    pub civilization: i16,
    #[pyo3(get, set)]
    pub icon_set: i16,
}

impl From<SoundItem> for PySoundItem {
    fn from(value: SoundItem) -> Self {
        Self {
            filename: value.filename.int_str,
            resource_id: value.resource_id,
            probability: value.probability,
            civilization: value.civilization,
            icon_set: value.icon_set,
        }
    }
}

impl From<PySoundItem> for SoundItem {
    fn from(value: PySoundItem) -> Self {
        Self {
            filename: PyDebugString::from(value.filename).into(),
            resource_id: value.resource_id,
            probability: value.probability,
            civilization: value.civilization,
            icon_set: value.icon_set,
        }
    }
}

#[pyclass(name = "Sound", module = "genieutils_rspy")]
pub struct PySound {
    #[pyo3(get, set)]
    pub id: i16,
    #[pyo3(get, set)]
    pub play_delay: i16,
    #[pyo3(get, set)]
    pub cache_time: i32,
    #[pyo3(get, set)]
    pub total_probability: i16,
    #[pyo3(get, set)]
    pub items: Py<PyList>,
}


impl TryFrom<PySound> for Sound {
    type Error = PyErr;
    fn try_from(value: PySound) -> PyResult<Self> {

        Python::with_gil(|py| -> PyResult<_> {
        let items: Vec<SoundItem> = value
            .items.extract(py)?
            .into_iter()
            .map(SoundItem::from)
            .collect();

            let result = Self {
                id: value.id,
                play_delay: value.play_delay,
                cache_time: value.cache_time,
                total_probability: value.total_probability,
                items: PyList::new(py, items)?.unbind(),
            };
            Ok(result)
        })
    }
}

use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct SoundItem {
    pub filename: DebugString,
    pub resource_id: i32,
    pub probability: i16,
    pub civilization: i16,
    pub icon_set: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct Sound {
    pub id: i16,
    pub play_delay: i16,
    #[br(temp)]
    #[bw(try_calc = items.len().try_into())]
    item_size: i16,
    pub cache_time: i32,
    pub total_probability: i16,
    #[br(count = item_size)]
    pub items: Vec<SoundItem>,
}

#[cfg(feature = "pyo3")]
mod python {
    use super::SoundItem;
    use super::Sound;
    use pyo3::prelude::*;
    use pyo3::types::PyList;
    #[pyclass(name = "SoundItem", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PySoundItem {
        pub filename: String,
        pub resource_id: i32,
        pub probability: i16,
        pub civilization: i16,
        pub icon_set: i16,
    }

    impl<'py> IntoPyObject<'py> for SoundItem {
        type Target = PySoundItem;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PySoundItem {
                filename: self.filename.int_str,
                resource_id: self.resource_id,
                probability: self.probability,
                civilization: self.civilization,
                icon_set: self.icon_set,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Sound", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PySound {
        pub id: i16,
        pub play_delay: i16,
        pub cache_time: i32,
        pub total_probability: i16,
        pub items: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for Sound {
        type Target = PySound;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let items = self.items.into_pyobject(py)?.downcast_into()?.unbind();

            let res = PySound {
                id: self.id,
                play_delay: self.play_delay,
                cache_time: self.cache_time,
                total_probability: self.total_probability,
                items: items,
            };
            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct EffectCommand {
    pub effect_type: u8,
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: f32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct Effect {
    pub name: DebugString,

    #[br(temp)]
    #[bw(try_calc = effect_commands.len().try_into())]
    effect_command_count: i16,

    #[br(count = effect_command_count)]
    pub effect_commands: Vec<EffectCommand>,
}

#[cfg(feature = "pyo3")]
mod python {
    use super::Effect;
    use super::EffectCommand;
    use pyo3::prelude::*;
    use pyo3::types::PyList;
    use pyo3::types::PyString;
    #[pyclass(name = "EffectCommand", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyEffectCommand {
        pub effect_type: u8,
        pub a: i16,
        pub b: i16,
        pub c: i16,
        pub d: f32,
    }

    impl<'py> IntoPyObject<'py> for EffectCommand {
        type Target = PyEffectCommand;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyEffectCommand {
                effect_type: self.effect_type,
                a: self.a,
                b: self.b,
                c: self.c,
                d: self.d,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "Effect", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyEffect {
        name: Py<PyString>,
        effect_commands: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for Effect {
        type Target = PyEffect;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyEffect {
                name: self.name.into_pyobject(py)?.unbind(),
                effect_commands: self
                    .effect_commands
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

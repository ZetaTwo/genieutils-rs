use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct ResearchResourceCost {
    pub cost_type: i16,
    pub amount: i16,
    pub flag: u8,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct Tech {
    pub required_techs: (i16, i16, i16, i16, i16, i16),
    pub resource_costs: (
        ResearchResourceCost,
        ResearchResourceCost,
        ResearchResourceCost,
    ),
    pub required_tech_count: i16,
    pub civ: i16,
    pub full_tech_mode: i16,
    pub research_location: i16,
    pub language_dll_name: i32,
    pub language_dll_description: i32,
    pub research_time: i16,
    pub effect_id: i16,
    pub tech_type: i16,
    pub icon_id: i16,
    pub button_id: u8,
    pub language_dll_help: i32,
    pub language_dll_tech_tree: i32,
    pub hot_key: i32,
    pub name: DebugString,
    pub repeatable: u8,
}

#[cfg(feature = "pyo3")]
mod python {
    use super::ResearchResourceCost;
    use super::Tech;

    use pyo3::prelude::*;
    use pyo3::types::PyString;
    use pyo3::types::PyTuple;

    #[pyclass(name = "ResearchResourceCost", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyResearchResourceCost {
        cost_type: i16,
        amount: i16,
        flag: u8,
    }

    impl<'py> IntoPyObject<'py> for ResearchResourceCost {
        type Target = PyResearchResourceCost;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyResearchResourceCost {
                cost_type: self.cost_type,
                amount: self.amount,
                flag: self.flag,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "Tech", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTech {
        required_techs: (i16, i16, i16, i16, i16, i16),
        resource_costs: Py<PyTuple>,
        required_tech_count: i16,
        civ: i16,
        full_tech_mode: i16,
        research_location: i16,
        language_dll_name: i32,
        language_dll_description: i32,
        research_time: i16,
        effect_id: i16,
        tech_type: i16,
        icon_id: i16,
        button_id: u8,
        language_dll_help: i32,
        language_dll_tech_tree: i32,
        hot_key: i32,
        name: Py<PyString>,
        repeatable: u8,
    }

    impl<'py> IntoPyObject<'py> for Tech {
        type Target = PyTech;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTech {
                required_techs: self.required_techs,
                resource_costs: self.resource_costs.into_pyobject(py)?.unbind(),
                required_tech_count: self.required_tech_count,
                civ: self.civ,
                full_tech_mode: self.full_tech_mode,
                research_location: self.research_location,
                language_dll_name: self.language_dll_name,
                language_dll_description: self.language_dll_description,
                research_time: self.research_time,
                effect_id: self.effect_id,
                tech_type: self.tech_type,
                icon_id: self.icon_id,
                button_id: self.button_id,
                language_dll_help: self.language_dll_help,
                language_dll_tech_tree: self.language_dll_tech_tree,
                hot_key: self.hot_key,
                name: self.name.into_pyobject(py)?.unbind(),
                repeatable: self.repeatable,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

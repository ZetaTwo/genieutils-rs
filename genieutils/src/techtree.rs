use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct Common {
    pub slots_used: i32,
    pub unit_research: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
    pub mode: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct TechTreeAge {
    pub id: i32,
    pub status: u8,

    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    pub buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    pub units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    pub techs: Vec<i32>,

    pub common: Common,
    pub num_building_levels: u8,
    pub buildings_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    pub group_length_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    pub max_age_length: u8,
    pub line_mode: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct BuildingConnection {
    pub id: i32,
    pub status: u8,
    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    pub buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    pub units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    pub techs: Vec<i32>,
    pub common: Common,
    pub location_in_age: u8,
    pub units_techs_total: (u8, u8, u8, u8, u8),
    pub units_techs_first: (u8, u8, u8, u8, u8),
    pub line_mode: i32,
    pub enabling_research: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct UnitConnection {
    pub id: i32,
    pub status: u8,
    pub upper_building: i32,
    pub common: Common,
    pub vertical_line: i32,
    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    pub units: Vec<i32>,
    pub location_in_age: i32,
    pub required_research: i32,
    pub line_mode: i32,
    pub enabling_research: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct ResearchConnection {
    pub id: i32,
    pub status: u8,
    pub upper_building: i32,
    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    pub buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    pub units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    pub techs: Vec<i32>,
    pub common: Common,
    pub vertical_line: i32,
    pub location_in_age: i32,
    pub line_mode: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct TechTree {
    #[br(temp)]
    #[bw(try_calc = tech_tree_ages.len().try_into())]
    age_count: u8,
    #[br(temp)]
    #[bw(try_calc = building_connections.len().try_into())]
    building_count: u8,
    #[br(temp)]
    #[bw(try_calc = unit_connections.len().try_into())]
    unit_count: u8,
    #[br(temp)]
    #[bw(try_calc = research_connections.len().try_into())]
    research_count: u8,

    pub total_unit_tech_groups: i32,

    #[br(count = age_count)]
    pub tech_tree_ages: Vec<TechTreeAge>,
    #[br(count = building_count)]
    pub building_connections: Vec<BuildingConnection>,
    #[br(count = unit_count)]
    pub unit_connections: Vec<UnitConnection>,
    #[br(count = research_count)]
    pub research_connections: Vec<ResearchConnection>,
}

#[cfg(feature = "pyo3")]
pub mod python {
    use super::BuildingConnection;
    use super::Common;
    use super::ResearchConnection;
    use super::TechTree;
    use super::TechTreeAge;
    use super::UnitConnection;
    use pyo3::prelude::*;
    use pyo3::types::PyList;
    #[pyclass(name = "Common", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyCommon {
        pub slots_used: i32,
        pub unit_research: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
        pub mode: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
    }

    impl<'py> IntoPyObject<'py> for Common {
        type Target = PyCommon;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyCommon {
                slots_used: self.slots_used,
                unit_research: self.unit_research,
                mode: self.mode,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "TechTreeAge", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTechTreeAge {
        id: i32,
        status: u8,
        buildings: Vec<i32>,
        units: Vec<i32>,
        techs: Vec<i32>,
        common: Py<PyCommon>,
        num_building_levels: u8,
        buildings_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
        group_length_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
        max_age_length: u8,
        line_mode: i32,
    }
    impl<'py> IntoPyObject<'py> for TechTreeAge {
        type Target = PyTechTreeAge;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTechTreeAge {
                id: self.id,
                status: self.status,
                buildings: self.buildings,
                units: self.units,
                techs: self.techs,
                common: self.common.into_pyobject(py)?.unbind(),
                num_building_levels: self.num_building_levels,
                buildings_per_zone: self.buildings_per_zone,
                group_length_per_zone: self.group_length_per_zone,
                max_age_length: self.max_age_length,
                line_mode: self.line_mode,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "BuildingConnection", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyBuildingConnection {
        id: i32,
        status: u8,
        buildings: Vec<i32>,
        units: Vec<i32>,
        techs: Vec<i32>,
        common: Py<PyCommon>,
        location_in_age: u8,
        units_techs_total: (u8, u8, u8, u8, u8),
        units_techs_first: (u8, u8, u8, u8, u8),
        line_mode: i32,
        enabling_research: i32,
    }
    impl<'py> IntoPyObject<'py> for BuildingConnection {
        type Target = PyBuildingConnection;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyBuildingConnection {
                id: self.id,
                status: self.status,
                buildings: self.buildings,
                units: self.units,
                techs: self.techs,
                common: self.common.into_pyobject(py)?.unbind(),
                location_in_age: self.location_in_age,
                units_techs_total: self.units_techs_total,
                units_techs_first: self.units_techs_first,
                line_mode: self.line_mode,
                enabling_research: self.enabling_research,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "UnitConnection", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyUnitConnection {
        id: i32,
        status: u8,
        upper_building: i32,
        common: Py<PyCommon>,
        vertical_line: i32,
        units: Vec<i32>,
        location_in_age: i32,
        required_research: i32,
        line_mode: i32,
        enabling_research: i32,
    }
    impl<'py> IntoPyObject<'py> for UnitConnection {
        type Target = PyUnitConnection;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyUnitConnection {
                id: self.id,
                status: self.status,
                upper_building: self.upper_building,
                common: self.common.into_pyobject(py)?.unbind(),
                vertical_line: self.vertical_line,
                units: self.units,
                location_in_age: self.location_in_age,
                required_research: self.required_research,
                line_mode: self.line_mode,
                enabling_research: self.enabling_research,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "ResearchConnection", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyResearchConnection {
        id: i32,
        status: u8,
        upper_building: i32,
        buildings: Vec<i32>,
        units: Vec<i32>,
        techs: Vec<i32>,
        common: Py<PyCommon>,
        vertical_line: i32,
        location_in_age: i32,
        line_mode: i32,
    }
    impl<'py> IntoPyObject<'py> for ResearchConnection {
        type Target = PyResearchConnection;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyResearchConnection {
                id: self.id,
                status: self.status,
                upper_building: self.upper_building,
                buildings: self.buildings,
                units: self.units,
                techs: self.techs,
                common: self.common.into_pyobject(py)?.unbind(),
                vertical_line: self.vertical_line,
                location_in_age: self.location_in_age,
                line_mode: self.line_mode,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
    #[pyclass(name = "TechTree", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTechTree {
        pub total_unit_tech_groups: i32,
        pub tech_tree_ages: Py<PyList>,
        pub building_connections: Py<PyList>,
        pub unit_connections: Py<PyList>,
        pub research_connections: Py<PyList>,
    }
    impl<'py> IntoPyObject<'py> for TechTree {
        type Target = PyTechTree;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTechTree {
                total_unit_tech_groups: self.total_unit_tech_groups,
                tech_tree_ages: self
                    .tech_tree_ages
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                building_connections: self
                    .building_connections
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                unit_connections: self
                    .unit_connections
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                research_connections: self
                    .research_connections
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct Common {
    slots_used: i32,
    unit_research: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
    mode: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct TechTreeAge {
    id: i32,
    status: u8,

    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    techs: Vec<i32>,

    common: Common,
    num_building_levels: u8,
    buildings_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    group_length_per_zone: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8),
    max_age_length: u8,
    line_mode: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct BuildingConnection {
    id: i32,
    status: u8,
    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    techs: Vec<i32>,
    common: Common,
    location_in_age: u8,
    units_techs_total: (u8, u8, u8, u8, u8),
    units_techs_first: (u8, u8, u8, u8, u8),
    line_mode: i32,
    enabling_research: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct UnitConnection {
    id: i32,
    status: u8,
    upper_building: i32,
    common: Common,
    vertical_line: i32,
    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    units: Vec<i32>,
    location_in_age: i32,
    required_research: i32,
    line_mode: i32,
    enabling_research: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct ResearchConnection {
    id: i32,
    status: u8,
    upper_building: i32,
    #[br(temp)]
    #[bw(try_calc = buildings.len().try_into())]
    buildings_count: u8,
    #[br(count = buildings_count)]
    buildings: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_count: u8,
    #[br(count = units_count)]
    units: Vec<i32>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_count: u8,
    #[br(count = techs_count)]
    techs: Vec<i32>,
    common: Common,
    vertical_line: i32,
    location_in_age: i32,
    line_mode: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
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

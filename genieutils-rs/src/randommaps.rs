use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct MapUnit {
    unit: i32,
    host_terrain: i32,
    group_placing: u8,
    scale_flag: u8,
    padding_1: i16,
    objects_per_group: i32,
    fluctuation: i32,
    groups_per_player: i32,
    group_arena: i32,
    player_id: i32,
    set_place_for_all_players: i32,
    min_distance_to_players: i32,
    max_distance_to_players: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct MapTerrain {
    proportion: i32,
    terrain: i32,
    clump_count: i32,
    edge_spacing: i32,
    placement_terrain: i32,
    clumpiness: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct MapLand {
    land_id: i32,
    terrain: u32,
    land_spacing: i32,
    base_size: i32,
    zone: u8,
    placement_type: u8,

    #[br(ignore)]
    #[bw(calc = 0)]
    padding_1: i16,

    base_x: i32,
    base_y: i32,
    land_proportion: u8,
    by_player_flag: u8,

    #[br(ignore)]
    #[bw(calc = 0)]
    padding_2: i16,

    start_area_radius: i32,
    terrain_edge_fade: i32,
    clumpiness: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct MapElevation {
    proportion: i32,
    terrain: i32,
    clump_count: i32,
    base_terrain: i32,
    base_elevation: i32,
    tile_spacing: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct MapInfo {
    map_id: i32,
    border_south_west: i32,
    border_north_west: i32,
    border_north_east: i32,
    border_south_east: i32,
    border_usage: i32,
    water_shape: i32,
    base_terrain: i32,
    land_coverage: i32,
    unused_id: i32,

    #[br(temp)]
    #[bw(try_calc = map_lands.len().try_into())]
    map_lands_size: u32,
    map_lands_ptr: i32,
    #[br(count = map_lands_size)]
    map_lands: Vec<MapLand>,

    #[br(temp)]
    #[bw(try_calc = map_terrains.len().try_into())]
    map_terrains_size: u32,
    map_terrains_ptr: i32,
    #[br(count = map_terrains_size)]
    map_terrains: Vec<MapTerrain>,

    #[br(temp)]
    #[bw(try_calc = map_units.len().try_into())]
    map_units_size: u32,
    map_units_ptr: i32,
    #[br(count = map_units_size)]
    map_units: Vec<MapUnit>,

    #[br(temp)]
    #[bw(try_calc = map_elevations.len().try_into())]
    map_elevations_size: u32,
    map_elevations_ptr: i32,
    #[br(count = map_elevations_size)]
    map_elevations: Vec<MapElevation>,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
#[bw(assert(map_info_1.len() == map_info_2.len(), "map_info lists lengths unmatched: {} != {}", map_info_1.len(), map_info_2.len()))]
pub struct RandomMaps {
    #[br(temp)]
    #[bw(try_calc = map_info_1.len().try_into())]
    random_map_count: u32,
    random_maps_ptr: i32,
    #[br(count = random_map_count)]
    map_info_1: Vec<MapInfo>,
    #[br(count = random_map_count)]
    map_info_2: Vec<MapInfo>,
}

#[cfg(feature = "pyo3")]
pub mod python {
    use super::MapElevation;
    use super::MapInfo;
    use super::MapLand;
    use super::MapTerrain;
    use super::MapUnit;
    use super::RandomMaps;

    use pyo3::prelude::*;
    use pyo3::types::PyList;

    #[pyclass(name = "MapUnit", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyMapUnit {
        unit: i32,
        host_terrain: i32,
        group_placing: u8,
        scale_flag: u8,
        padding_1: i16,
        objects_per_group: i32,
        fluctuation: i32,
        groups_per_player: i32,
        group_arena: i32,
        player_id: i32,
        set_place_for_all_players: i32,
        min_distance_to_players: i32,
        max_distance_to_players: i32,
    }
    impl<'py> IntoPyObject<'py> for MapUnit {
        type Target = PyMapUnit;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyMapUnit {
                unit: self.unit,
                host_terrain: self.host_terrain,
                group_placing: self.group_placing,
                scale_flag: self.scale_flag,
                padding_1: self.padding_1,
                objects_per_group: self.objects_per_group,
                fluctuation: self.fluctuation,
                groups_per_player: self.groups_per_player,
                group_arena: self.group_arena,
                player_id: self.player_id,
                set_place_for_all_players: self.set_place_for_all_players,
                min_distance_to_players: self.min_distance_to_players,
                max_distance_to_players: self.max_distance_to_players,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "MapTerrain", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyMapTerrain {
        proportion: i32,
        terrain: i32,
        clump_count: i32,
        edge_spacing: i32,
        placement_terrain: i32,
        clumpiness: i32,
    }

    impl<'py> IntoPyObject<'py> for MapTerrain {
        type Target = PyMapTerrain;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyMapTerrain {
                proportion: self.proportion,
                terrain: self.terrain,
                clump_count: self.clump_count,
                edge_spacing: self.edge_spacing,
                placement_terrain: self.placement_terrain,
                clumpiness: self.clumpiness,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "MapLand", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyMapLand {
        land_id: i32,
        terrain: u32,
        land_spacing: i32,
        base_size: i32,
        zone: u8,
        placement_type: u8,
        base_x: i32,
        base_y: i32,
        land_proportion: u8,
        by_player_flag: u8,
        start_area_radius: i32,
        terrain_edge_fade: i32,
        clumpiness: i32,
    }

    impl<'py> IntoPyObject<'py> for MapLand {
        type Target = PyMapLand;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyMapLand {
                land_id: self.land_id,
                terrain: self.terrain,
                land_spacing: self.land_spacing,
                base_size: self.base_size,
                zone: self.zone,
                placement_type: self.placement_type,
                base_x: self.base_x,
                base_y: self.base_y,
                land_proportion: self.land_proportion,
                by_player_flag: self.by_player_flag,
                start_area_radius: self.start_area_radius,
                terrain_edge_fade: self.terrain_edge_fade,
                clumpiness: self.clumpiness,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "MapElevation", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyMapElevation {
        proportion: i32,
        terrain: i32,
        clump_count: i32,
        base_terrain: i32,
        base_elevation: i32,
        tile_spacing: i32,
    }

    impl<'py> IntoPyObject<'py> for MapElevation {
        type Target = PyMapElevation;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyMapElevation {
                proportion: self.proportion,
                terrain: self.terrain,
                clump_count: self.clump_count,
                base_terrain: self.base_terrain,
                base_elevation: self.base_elevation,
                tile_spacing: self.tile_spacing,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "MapInfo", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyMapInfo {
        map_id: i32,
        border_south_west: i32,
        border_north_west: i32,
        border_north_east: i32,
        border_south_east: i32,
        border_usage: i32,
        water_shape: i32,
        base_terrain: i32,
        land_coverage: i32,
        unused_id: i32,

        map_lands_ptr: i32,
        map_lands: Py<PyList>,

        map_terrains_ptr: i32,
        map_terrains: Py<PyList>,

        map_units_ptr: i32,
        map_units: Py<PyList>,

        map_elevations_ptr: i32,
        map_elevations: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for MapInfo {
        type Target = PyMapInfo;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyMapInfo {
                map_id: self.map_id,
                border_north_west: self.border_north_west,
                border_south_west: self.border_south_west,
                border_north_east: self.border_north_east,
                border_south_east: self.border_south_east,
                border_usage: self.border_usage,
                water_shape: self.water_shape,
                base_terrain: self.base_terrain,
                land_coverage: self.land_coverage,
                unused_id: self.unused_id,
                map_lands_ptr: self.map_lands_ptr,
                map_lands: self.map_lands.into_pyobject(py)?.downcast_into()?.unbind(),
                map_terrains_ptr: self.map_terrains_ptr,
                map_terrains: self
                    .map_terrains
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                map_units_ptr: self.map_units_ptr,
                map_units: self.map_units.into_pyobject(py)?.downcast_into()?.unbind(),
                map_elevations_ptr: self.map_elevations_ptr,
                map_elevations: self
                    .map_elevations
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "RandomMaps", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyRandomMaps {
        random_maps_ptr: i32,
        map_info_1: Py<PyList>,
        map_info_2: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for RandomMaps {
        type Target = PyRandomMaps;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyRandomMaps {
                random_maps_ptr: self.random_maps_ptr,
                map_info_1: self.map_info_1.into_pyobject(py)?.downcast_into()?.unbind(),
                map_info_2: self.map_info_2.into_pyobject(py)?.downcast_into()?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

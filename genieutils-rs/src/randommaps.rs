use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct MapUnit {
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
struct MapTerrain {
    proportion: i32,
    terrain: i32,
    clump_count: i32,
    edge_spacing: i32,
    placement_terrain: i32,
    clumpiness: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct MapLand {
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
struct MapElevation {
    proportion: i32,
    terrain: i32,
    clump_count: i32,
    base_terrain: i32,
    base_elevation: i32,
    tile_spacing: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct MapInfo {
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

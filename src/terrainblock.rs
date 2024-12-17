use binrw::binrw;

use crate::common::DebugString;
use crate::common::TERRAIN_COUNT;
use crate::common::TERRAIN_UNITS_SIZE;
use crate::common::TILE_TYPE_COUNT;

#[binrw]
struct FrameData {
    frame_count: i16,
    angle_count: i16,
    shape_id: i16,
}

#[binrw]
struct Terrain {
    enabled: u8,
    random: u8,
    is_water: u8,
    hide_in_editor: u8,
    string_id: i32,
    name: DebugString,
    name_2: DebugString,
    slp: i32,
    shape_ptr: i32,
    sound_id: i32,
    wwise_sound_id: u32,
    wwise_sound_stop_id: u32,
    blend_priority: i32,
    blend_type: i32,
    overlay_mask_name: DebugString,
    colors: (u8, u8, u8),
    cliff_colors: (u8, u8),
    passable_terrain: u8,
    impassable_terrain: u8,
    is_animated: u8,
    animation_frames: i16,
    pause_frames: i16,
    interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: u8,
    drawn: u8,

    #[br(count = TILE_TYPE_COUNT)]
    frame_data: Vec<FrameData>,
    terrain_to_draw: i16,
    terrain_dimensions: (i16, i16),

    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_masked_density: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_id: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_density: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_centering: Vec<u8>,

    number_of_terrain_units_used: i16,
    phantom: i16,
}

#[binrw]
struct TileSize {
    width: i16,
    height: i16,
    delta_y: i16,
}

#[binrw]
pub struct TerrainBlock {
    virtual_function_ptr: u32,
    map_pointer: u32,
    map_width: i32,
    map_height: i32,
    world_width: i32,
    world_height: i32,

    #[br(count = TILE_TYPE_COUNT)]
    tile_sizes: Vec<TileSize>,
    padding_ts: i16,
    #[br(count = TERRAIN_COUNT)]
    terrains: Vec<Terrain>,
    map_min_x: f32,
    map_min_y: f32,
    map_max_x: f32,
    map_max_y: f32,
    map_max_x_plus_1: f32,
    map_max_y_plus_1: f32,
    terrains_used_2: i16,
    borders_used: i16,
    max_terrain: i16,
    tile_width: i16,
    tile_height: i16,
    tile_half_height: i16,
    tile_half_width: i16,
    elev_height: i16,
    cur_row: i16,
    cur_col: i16,
    block_beg_row: i16,
    block_end_row: i16,
    block_beg_col: i16,
    block_end_col: i16,

    search_map_ptr: u32,
    search_map_rows_ptr: u32,
    any_frame_change: u8,
    map_visible_flag: u8,
    fog_flag: u8,
}

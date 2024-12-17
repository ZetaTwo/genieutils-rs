use binrw::binrw;

#[binrw]
struct TerrainPassGraphic {
    exit_tile_sprite_id: i32,
    enter_tile_sprite_id: i32,
    walk_tile_sprite_id: i32,
    walk_sprite_rate: i32,
}

// TODO: from_bytes_with_count(cls, content: ByteHandler, terrain_count: int)
/*#[binrw]
struct TerrainRestriction {
    passable_buildable_dmg_multiplier: Vec<f32>,
    terrain_pass_graphics: Vec<TerrainPassGraphic>,
}*/

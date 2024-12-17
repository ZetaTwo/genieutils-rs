use binrw::binrw;

#[binrw]
struct TerrainPassGraphic {
    exit_tile_sprite_id: i32,
    enter_tile_sprite_id: i32,
    walk_tile_sprite_id: i32,
    walk_sprite_rate: i32,
}

#[binrw]
#[br(import(terrain_count: usize))]
#[bw(assert(passable_buildable_dmg_multiplier.len() == terrain_pass_graphics.len(), "terrain restriciton lists lengths unmatched: {} != {}", passable_buildable_dmg_multiplier.len(), terrain_pass_graphics.len()))]

pub struct TerrainRestriction {
    #[br(count = terrain_count)]
    pub passable_buildable_dmg_multiplier: Vec<f32>,
    #[br(count = terrain_count)]
    pub terrain_pass_graphics: Vec<TerrainPassGraphic>,
}

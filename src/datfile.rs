use binrw::binrw;

use crate::civ::Civ;
use crate::effect::Effect;
use crate::graphic::Graphic;
use crate::playercolour::PlayerColour;
use crate::randommaps::RandomMaps;
use crate::sound::Sound;
use crate::tech::Tech;
use crate::techtree::TechTree;
use crate::terrainblock::TerrainBlock;
use crate::terrainrestrictions::TerrainRestriction;
use crate::unitheaders::UnitHeaders;

#[binrw]
#[bw(assert(float_ptr_terrain_tables.len() == terrain_pass_graphic_pointers.len() && terrain_pass_graphic_pointers.len() == terrain_restrictions.len(), "terrain_tables lists lengths unmatched: {} != {} != {}", float_ptr_terrain_tables.len(), terrain_pass_graphic_pointers.len(), terrain_restrictions.len()))]
struct DatFile {
    #[br(count = 8)]
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x))]
    #[bw(map = |x: &String| x.as_bytes())]
    version: String,

    #[br(temp)]
    #[bw(try_calc = float_ptr_terrain_tables.len().try_into())]
    terrain_restrictions_size: i16,

    terrains_used_1: i16, // TODO: check this

    #[br(count = terrain_restrictions_size)]
    float_ptr_terrain_tables: Vec<u32>,

    #[br(count = terrain_restrictions_size)]
    terrain_pass_graphic_pointers: Vec<u32>,

    #[br(count = terrain_restrictions_size)]
    terrain_restrictions: Vec<TerrainRestriction>,

    #[br(temp)]
    #[bw(try_calc = player_colours.len().try_into())]
    player_colours_size: i16,
    #[br(count = player_colours_size)]
    player_colours: Vec<PlayerColour>,

    #[br(temp)]
    #[bw(try_calc = sounds.len().try_into())]
    sounds_size: i16,
    #[br(count = sounds_size)]
    sounds: Vec<Sound>,

    #[br(temp)]
    #[bw(try_calc = graphics.len().try_into())]
    graphics_size: i16,

    // #[br(count = graphics_size)]
    // TODO: graphic_pointers
    #[br(count = graphics_size)]
    graphics: Vec<Option<Graphic>>,

    terrain_block: TerrainBlock,
    random_maps: RandomMaps,

    #[br(temp)]
    #[bw(try_calc = effects.len().try_into())]
    effects_size: i32,
    #[br(count = effects_size)]
    effects: Vec<Effect>,

    #[br(temp)]
    #[bw(try_calc = unit_headers.len().try_into())]
    unit_headers_size: i32,
    #[br(count = unit_headers_size)]
    unit_headers: Vec<UnitHeaders>,

    #[br(temp)]
    #[bw(try_calc = civs.len().try_into())]
    civs_size: i16,
    #[br(count = civs_size)]
    civs: Vec<Civ>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_size: i16,
    #[br(count = techs_size)]
    techs: Vec<Tech>,

    time_slice: u32,
    unit_kill_rate: u32,
    unit_kill_total: u32,
    unit_hit_point_rate: u32,
    unit_hit_point_total: u32,
    razing_kill_rate: u32,
    razing_kill_total: u32,
    tech_tree: TechTree,
}

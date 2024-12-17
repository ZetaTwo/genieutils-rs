use binrw::binrw;
use binrw::helpers::args_iter_with;
use binrw::BinRead;

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
use crate::versions::Version;

#[binrw]
#[bw(assert(float_ptr_terrain_tables.len() == terrain_pass_graphic_pointers.len() && terrain_pass_graphic_pointers.len() == terrain_restrictions.len(), "terrain_tables lists lengths unmatched: {} != {} != {}", float_ptr_terrain_tables.len(), terrain_pass_graphic_pointers.len(), terrain_restrictions.len()))]
struct DatFile {
    #[br(count = 8)]
    #[br(try_map = |x: Vec<u8>| {
        String::from_utf8(x)
            .map_err(|err|std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?
            .try_into()
    })]
    #[bw(map = |x: &Version| x.to_string().bytes().take(8))]
    version: Version,

    #[br(temp)]
    #[bw(try_calc = float_ptr_terrain_tables.len().try_into())]
    terrain_restrictions_size: i16,

    #[br(temp)]
    #[br(try_map = |x: i16| x.try_into())]
    // TODO: is this truly correct?
    #[bw(calc = {
        terrain_restrictions
          .get(0)
          .map(|restriction: &TerrainRestriction| restriction.passable_buildable_dmg_multiplier.len())
          .unwrap_or(0)
    })]
    terrains_used_1: usize,

    #[br(count = terrain_restrictions_size)]
    float_ptr_terrain_tables: Vec<u32>,

    #[br(count = terrain_restrictions_size)]
    terrain_pass_graphic_pointers: Vec<u32>,

    #[br(
        count = terrain_restrictions_size,
        args { inner: (terrains_used_1,)  }
    )]
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

    #[br(count = graphics_size)]
    graphic_pointers: Vec<i32>,
    #[br(parse_with = args_iter_with(&graphic_pointers,
    |reader, endian, &pointer| {
        if pointer == 0 {
            Ok(None)
        } else {
            <Graphic as BinRead>::read_options(reader, endian, ()).map(|x|Some(x))
        }
    }
    ))]
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

    #[br(
        count = civs_size,
        args { inner: (version,)  }
    )]
    #[bw(args { inner: (version,) })]
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

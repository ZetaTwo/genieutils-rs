use std::io::Read;

use binrw::helpers::args_iter_with;
use binrw::{binrw, BinResult};
use binrw::{BinRead, BinWrite};
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Cursor;

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
#[brw(little)]
#[bw(assert(float_ptr_terrain_tables.len() == terrain_pass_graphic_pointers.len() && terrain_pass_graphic_pointers.len() == terrain_restrictions.len(), "terrain_tables lists lengths unmatched: {} != {} != {}", float_ptr_terrain_tables.len(), terrain_pass_graphic_pointers.len(), terrain_restrictions.len()))]
pub struct DatFile {
    pub version: Version,

    #[br(temp)]
    #[bw(try_calc = float_ptr_terrain_tables.len().try_into())]
    terrain_restrictions_size: i16,

    #[br(temp)]
    #[br(try_map = |x: i16| x.try_into())]
    // TODO: is this truly correct?
    #[bw(calc = {
        terrain_restrictions
          .first()
          .map(|restriction: &TerrainRestriction| restriction.passable_buildable_dmg_multiplier.len())
          .unwrap_or(0)
    })]
    #[bw(try_map = |x: usize| TryInto::<i16>::try_into(x))]
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
    #[bw(try_calc = graphic_pointers.len().try_into())]
    graphics_size: i16,

    #[br(count = graphics_size)]
    graphic_pointers: Vec<i32>,
    #[br(parse_with = args_iter_with(&graphic_pointers,
    |reader, endian, &pointer| {
        if pointer == 0 {
            Ok(None)
        } else {
            <Graphic as BinRead>::read_options(reader, endian, ()).map(Some)
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
    #[bw(args (*version,))]
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

impl DatFile {
    pub fn parse_compressed(data: &Vec<u8>) -> BinResult<Self> {
        let deflated = Self::decompress(data)?;
        let mut stream = Cursor::new(deflated);
        Self::read(&mut stream)
    }

    pub fn parse(data: &Vec<u8>) -> BinResult<Self> {
        let mut stream = Cursor::new(data);
        Self::read(&mut stream)
    }

    pub fn decompress(data: &Vec<u8>) -> Result<Vec<u8>, binrw::Error> {
        let mut deflater = DeflateDecoder::new(&data[..]);
        let mut deflated: Vec<u8> = Vec::new();
        deflater.read_to_end(&mut deflated)?;
        Ok(deflated)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, binrw::Error> {
        let mut writer = Cursor::new(Vec::new());
        self.write(&mut writer)?;
        let serialized = writer.into_inner();
        Ok(serialized)
    }

    pub fn pack(&self) -> Result<Vec<u8>, binrw::Error> {
        let serialized = self.serialize()?;
        let mut inflater = DeflateEncoder::new(Vec::new(), Compression::default());
        inflater.write_all(&serialized)?;
        let inflated = inflater.finish()?;

        Ok(inflated)
    }
}

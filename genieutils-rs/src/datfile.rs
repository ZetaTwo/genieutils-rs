use std::io::Read;

use binrw::helpers::args_iter_with;
use binrw::{binrw, BinResult};
use binrw::{BinRead, BinWrite};
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Cursor;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/*#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]*/
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
#[bw(assert(float_ptr_terrain_tables.len() == terrain_pass_graphic_pointers.len() && terrain_pass_graphic_pointers.len() == terrain_restrictions.len(), "terrain_tables lists lengths unmatched: {} != {} != {}", float_ptr_terrain_tables.len(), terrain_pass_graphic_pointers.len(), terrain_restrictions.len()))]
pub struct DatFile {
    pub version: Version,

    #[br(temp)]
    #[bw(try_calc = float_ptr_terrain_tables.len().try_into())]
    pub terrain_restrictions_size: i16,

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
    pub terrains_used_1: usize,

    #[br(count = terrain_restrictions_size)]
    pub float_ptr_terrain_tables: Vec<u32>,

    #[br(count = terrain_restrictions_size)]
    pub terrain_pass_graphic_pointers: Vec<u32>,

    #[br(
        count = terrain_restrictions_size,
        args { inner: (terrains_used_1,)  }
    )]
    pub terrain_restrictions: Vec<TerrainRestriction>,

    #[br(temp)]
    #[bw(try_calc = player_colours.len().try_into())]
    player_colours_size: i16,
    #[br(count = player_colours_size)]
    pub player_colours: Vec<PlayerColour>,

    #[br(temp)]
    #[bw(try_calc = sounds.len().try_into())]
    sounds_size: i16,
    #[br(count = sounds_size)]
    pub sounds: Vec<Sound>,

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
    pub graphics: Vec<Option<Graphic>>,

    pub terrain_block: TerrainBlock,
    pub random_maps: RandomMaps,

    #[br(temp)]
    #[bw(try_calc = effects.len().try_into())]
    effects_size: i32,
    #[br(count = effects_size)]
    pub effects: Vec<Effect>,

    #[br(temp)]
    #[bw(try_calc = unit_headers.len().try_into())]
    unit_headers_size: i32,
    #[br(count = unit_headers_size)]
    pub unit_headers: Vec<UnitHeaders>,

    #[br(temp)]
    #[bw(try_calc = civs.len().try_into())]
    civs_size: i16,

    #[br(
        count = civs_size,
        args { inner: (version,)  }
    )]
    #[bw(args (*version,))]
    pub civs: Vec<Civ>,

    #[br(temp)]
    #[bw(try_calc = techs.len().try_into())]
    techs_size: i16,
    #[br(count = techs_size)]
    pub techs: Vec<Tech>,

    pub time_slice: u32,
    pub unit_kill_rate: u32,
    pub unit_kill_total: u32,
    pub unit_hit_point_rate: u32,
    pub unit_hit_point_total: u32,
    pub razing_kill_rate: u32,
    pub razing_kill_total: u32,
    pub tech_tree: TechTree,
}

impl DatFile {
    pub fn parse_compressed(data: &[u8]) -> BinResult<Self> {
        let deflated = Self::decompress(data)?;
        let mut stream = Cursor::new(deflated);
        Self::read(&mut stream)
    }

    pub fn parse(data: &Vec<u8>) -> BinResult<Self> {
        let mut stream = Cursor::new(data);
        Self::read(&mut stream)
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, binrw::Error> {
        let mut deflater = DeflateDecoder::new(data);
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
/*
#[cfg(feature = "pyo3")]
mod python {
    use super::DatFile;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;




    #[pymethods]
    impl DatFile {
        #[staticmethod]
        #[pyo3(name="parse_compressed")]
        fn py_parse_compressed(data: &[u8]) -> PyResult<Self> {
            let datfile = DatFile::parse_compressed(data)
                .map_err(|err| PyValueError::new_err(err.to_string()))?;
            Ok(datfile)
        }

        #[staticmethod]
        #[pyo3(name="parse")]
        fn py_parse(data: Vec<u8>) -> PyResult<Self> {
            let datfile =
                DatFile::parse(&data).map_err(|err| PyValueError::new_err(err.to_string()))?;
            Ok(datfile)
        }

        #[staticmethod]
        #[pyo3(name="decompress")]
        fn py_decompress(data: &[u8]) -> PyResult<Vec<u8>> {
            let data =
                DatFile::decompress(data).map_err(|err| PyValueError::new_err(err.to_string()))?;
            Ok(data)
        }

        #[pyo3(name="serialize")]
        fn py_serialize(&self) -> PyResult<Vec<u8>> {
            let data = self
                .serialize()
                .map_err(|err| PyValueError::new_err(err.to_string()))?;
            Ok(data)
        }

        #[pyo3(name="pack")]
        fn py_pack(&self) -> PyResult<Vec<u8>> {
            let data = self
                .pack()
                .map_err(|err| PyValueError::new_err(err.to_string()))?;
            Ok(data)
        }

        #[getter]
        #[pyo3(name="version")]
        fn py_version(&self) -> PyResult<u8> {
            Ok(self.version as u8)
        }
    }
}
*/

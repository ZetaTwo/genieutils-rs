pub mod civ;
pub mod common;
pub mod datfile;
pub mod effect;
pub mod graphic;
pub mod playercolour;
pub mod randommaps;
pub mod sound;
pub mod task;
pub mod tech;
pub mod techtree;
pub mod terrainblock;
pub mod terrainrestrictions;
pub mod unit;
pub mod unitheaders;
pub mod versions;

#[cfg(test)]
mod tests {
    use super::*;
    use datfile::DatFile;

    #[test]
    fn parse_datfile() {
        let data: Vec<u8> = std::fs::read("test/empires2_x2_p1.dat").unwrap();
        let datfile = DatFile::parse_compressed(&data).unwrap();

        println!("Version: {}", datfile.version);
    }

    #[test]
    fn parse_serialize_datfile() {
        let data: Vec<u8> = std::fs::read("test/empires2_x2_p1.dat").unwrap();

        let inflated = DatFile::decompress(&data).unwrap();
        let datfile = DatFile::parse(&inflated).unwrap();
        let serialized = datfile.serialize().unwrap();

        std::fs::write("test/cmp_inflated.dat", &inflated).unwrap();
        std::fs::write("test/cmp_serialized.dat", &serialized).unwrap();

        assert_eq!(
            inflated, serialized,
            "serialized data not equal to original data"
        );
    }
}

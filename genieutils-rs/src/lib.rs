mod civ;
mod common;
pub mod datfile;
mod effect;
mod graphic;
mod playercolour;
mod randommaps;
mod sound;
mod task;
mod tech;
mod techtree;
mod terrainblock;
mod terrainrestrictions;
mod unit;
mod unitheaders;
mod versions;

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

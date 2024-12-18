use genieutils::datfile::DatFile;

fn main() {
    let data: Vec<u8> = std::fs::read("test/empires2_x2_p1.dat").unwrap();

    let inflated = DatFile::decompress(&data).unwrap();
    let datfile = DatFile::parse(&inflated).unwrap();
    //let serialized = datfile.serialize().unwrap();

    //println!("Round trip success: {}", inflated == serialized);
    println!("Some data: {}", datfile.tech_tree.total_unit_tech_groups);
}

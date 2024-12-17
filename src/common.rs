use binrw::binrw;

#[binrw]
#[br(assert(temp_size == 0x0A60, "DebugString temp_size invalid: {}", temp_size))]
pub struct DebugString {
    #[br(temp)]
    #[bw(calc = 0x0A60)]
    temp_size: u16,

    size: u16,

    #[br(count = size)]
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x))]
    #[bw(map = |x: &String| x.as_bytes())]
    int_str: String,
}

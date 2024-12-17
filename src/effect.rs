use binrw::binrw;

use crate::common::DebugString;

#[binrw]
struct EffectCommand {
    r#type: u8,
    a: i16,
    b: i16,
    c: i16,
    d: f32,
}

#[binrw]
struct Effect {
    name: DebugString,

    #[br(temp)]
    #[bw(try_calc = effect_commands.len().try_into())]
    effect_command_count: i16,

    #[br(count = effect_command_count)]
    effect_commands: Vec<EffectCommand>,
}

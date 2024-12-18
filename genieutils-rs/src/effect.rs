use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct EffectCommand {
    r#type: u8,
    a: i16,
    b: i16,
    c: i16,
    d: f32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Effect {
    name: DebugString,

    #[br(temp)]
    #[bw(try_calc = effect_commands.len().try_into())]
    effect_command_count: i16,

    #[br(count = effect_command_count)]
    effect_commands: Vec<EffectCommand>,
}

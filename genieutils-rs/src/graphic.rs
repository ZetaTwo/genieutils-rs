use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
struct GraphicDelta {
    graphic_id: i16,
    padding_1: i16,
    sprite_ptr: i32,
    offset_x: i16,
    offset_y: i16,
    display_angle: i16,
    padding_2: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
struct GraphicAngleSound {
    frame_num: i16,
    sound_id: i16,
    wwise_sound_id: i32,
    frame_num_2: i16,
    sound_id_2: i16,
    wwise_sound_id_2: i32,
    frame_num_3: i16,
    sound_id_3: i16,
    wwise_sound_id_3: i32,
}

#[binrw]
#[brw(little)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "genieutils_rspy", get_all, set_all)
)]
#[derive(Clone)]
pub struct Graphic {
    name: DebugString,
    file_name: DebugString,
    particle_effect_name: DebugString,
    slp: i32,
    is_loaded: u8,
    old_color_flag: u8,
    layer: u8,
    player_color: i16,
    transparent_selection: u8,
    coordinates: (i16, i16, i16, i16),

    #[br(temp)]
    #[bw(try_calc = deltas.len().try_into())]
    delta_count: i16,

    sound_id: i16,
    wwise_sound_id: i32,
    angle_sounds_used: u8,
    frame_count: i16,

    // TODO: Is this correct? Why is the count set even when unused?
    /*#[br(temp)]
    #[bw(try_calc = angle_sounds.len().try_into())]*/
    angle_count: i16,

    speed_multiplier: f32,
    frame_duration: f32,
    replay_delay: f32,
    sequence_type: u8,
    id: i16,
    mirroring_mode: u8,
    editor_flag: u8,

    #[br(count = delta_count)]
    deltas: Vec<GraphicDelta>,

    #[br(count = angle_count, if(angle_sounds_used != 0))]
    angle_sounds: Vec<GraphicAngleSound>,
}

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Task {
    task_type: i16,
    id: i16,
    is_default: u8,
    action_type: i16,
    class_id: i16,
    unit_id: i16,
    terrain_id: i16,
    resource_in: i16,
    resource_multiplier: i16,
    resource_out: i16,
    unused_resource: i16,
    work_value_1: f32,
    work_value_2: f32,
    work_range: f32,
    auto_search_targets: u8,
    search_wait_time: f32,
    enable_targeting: u8,
    combat_level_flag: u8,
    gather_type: i16,
    work_flag_2: i16,
    target_diplomacy: u8,
    carry_check: u8,
    pick_for_construction: u8,
    moving_graphic_id: i16,
    proceeding_graphic_id: i16,
    working_graphic_id: i16,
    carrying_graphic_id: i16,
    resource_gathering_sound_id: i16,
    resource_deposit_sound_id: i16,
    wwise_resource_gathering_sound_id: i32,
    wwise_resource_deposit_sound_id: i32,
}

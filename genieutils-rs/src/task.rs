use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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

#[cfg(feature = "pyo3")]
mod python {
    use super::Task;
    use pyo3::prelude::*;

    #[pyclass(name = "Task", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTask {
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

    impl<'py> IntoPyObject<'py> for Task {
        type Target = PyTask;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTask {
                task_type: self.task_type,
                id: self.id,
                is_default: self.is_default,
                action_type: self.action_type,
                class_id: self.class_id,
                unit_id: self.unit_id,
                terrain_id: self.terrain_id,
                resource_in: self.resource_in,
                resource_multiplier: self.resource_multiplier,
                resource_out: self.resource_out,
                unused_resource: self.unused_resource,
                work_value_1: self.work_value_1,
                work_value_2: self.work_value_2,
                work_range: self.work_range,
                auto_search_targets: self.auto_search_targets,
                search_wait_time: self.search_wait_time,
                enable_targeting: self.enable_targeting,
                combat_level_flag: self.combat_level_flag,
                gather_type: self.gather_type,
                work_flag_2: self.work_flag_2,
                target_diplomacy: self.target_diplomacy,
                carry_check: self.carry_check,
                pick_for_construction: self.pick_for_construction,
                moving_graphic_id: self.moving_graphic_id,
                proceeding_graphic_id: self.proceeding_graphic_id,
                working_graphic_id: self.working_graphic_id,
                carrying_graphic_id: self.carrying_graphic_id,
                resource_gathering_sound_id: self.resource_gathering_sound_id,
                resource_deposit_sound_id: self.resource_deposit_sound_id,
                wwise_resource_gathering_sound_id: self.wwise_resource_gathering_sound_id,
                wwise_resource_deposit_sound_id: self.wwise_resource_deposit_sound_id,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

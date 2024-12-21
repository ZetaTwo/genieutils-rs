use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::task::Task;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
struct TaskList {
    #[br(temp)]
    #[bw(try_calc = task_list.len().try_into())]
    task_count: i16,

    #[br(count = task_count)]
    task_list: Vec<Task>,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct UnitHeaders {
    exists: u8,

    #[br(if(exists != 0, None))]
    #[bw(if(*exists != 0))]
    task_list: Option<TaskList>,
}

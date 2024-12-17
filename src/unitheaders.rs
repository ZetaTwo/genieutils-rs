use binrw::binrw;

use crate::task::Task;

#[binrw]
struct TaskList {
    #[br(temp)]
    #[bw(try_calc = task_list.len().try_into())]
    task_count: i16,

    #[br(count = task_count)]
    task_list: Vec<Task>,
}

#[binrw]
pub struct UnitHeaders {
    exists: u8,

    #[br(if(exists != 0, None))]
    #[bw(if(*exists != 0))]
    task_list: Option<TaskList>,
}
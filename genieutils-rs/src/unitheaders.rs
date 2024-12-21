use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::task::Task;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct TaskList {
    #[br(temp)]
    #[bw(try_calc = task_list.len().try_into())]
    task_count: i16,

    #[br(count = task_count)]
    task_list: Vec<Task>,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct UnitHeaders {
    exists: u8,

    #[br(if(exists != 0, None))]
    #[bw(if(*exists != 0))]
    task_list: Option<TaskList>,
}

#[cfg(feature = "pyo3")]
pub mod python {
    use super::TaskList;

    use super::UnitHeaders;

    use pyo3::prelude::*;
    use pyo3::types::PyAny;
    use pyo3::types::PyList;

    #[pyclass(name = "TaskList", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTaskList {
        task_list: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for TaskList {
        type Target = PyTaskList;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTaskList {
                task_list: self.task_list.into_pyobject(py)?.downcast_into()?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "UnitHeaders", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyUnitHeaders {
        exists: u8,
        task_list: Py<PyAny>,
    }
    impl<'py> IntoPyObject<'py> for UnitHeaders {
        type Target = PyUnitHeaders;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyUnitHeaders {
                exists: self.exists,
                task_list: self.task_list.into_pyobject(py)?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

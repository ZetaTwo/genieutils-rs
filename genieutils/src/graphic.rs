use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct GraphicDelta {
    pub graphic_id: i16,
    pub padding_1: i16,
    pub sprite_ptr: i32,
    pub offset_x: i16,
    pub offset_y: i16,
    pub display_angle: i16,
    pub padding_2: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct GraphicAngleSound {
    pub frame_num: i16,
    pub sound_id: i16,
    pub wwise_sound_id: i32,
    pub frame_num_2: i16,
    pub sound_id_2: i16,
    pub wwise_sound_id_2: i32,
    pub frame_num_3: i16,
    pub sound_id_3: i16,
    pub wwise_sound_id_3: i32,
}

#[binrw]
#[brw(little)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct Graphic {
    pub name: DebugString,
    pub file_name: DebugString,
    pub particle_effect_name: DebugString,
    pub slp: i32,
    pub is_loaded: u8,
    pub old_color_flag: u8,
    pub layer: u8,
    pub player_color: i16,
    pub transparent_selection: u8,
    pub coordinates: (i16, i16, i16, i16),

    #[br(temp)]
    #[bw(try_calc = deltas.len().try_into())]
    delta_count: i16,

    pub sound_id: i16,
    pub wwise_sound_id: i32,
    pub angle_sounds_used: u8,
    pub frame_count: i16,

    // TODO: Is this correct? Why is the count set even when unused?
    /*#[br(temp)]
    #[bw(try_calc = angle_sounds.len().try_into())]*/
    pub angle_count: i16,

    pub speed_multiplier: f32,
    pub frame_duration: f32,
    pub replay_delay: f32,
    pub sequence_type: u8,
    pub id: i16,
    pub mirroring_mode: u8,
    pub editor_flag: u8,

    #[br(count = delta_count)]
    pub deltas: Vec<GraphicDelta>,

    #[br(count = angle_count, if(angle_sounds_used != 0))]
    pub angle_sounds: Vec<GraphicAngleSound>,
}

#[cfg(feature = "pyo3")]
mod python {
    use pyo3::prelude::*;
    use pyo3::types::{PyList, PyString};

    use super::{Graphic, GraphicAngleSound, GraphicDelta};

    #[pyclass(name = "GraphicDelta", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyGraphicDelta {
        graphic_id: i16,
        padding_1: i16,
        sprite_ptr: i32,
        offset_x: i16,
        offset_y: i16,
        display_angle: i16,
        padding_2: i16,
    }

    impl<'py> IntoPyObject<'py> for GraphicDelta {
        type Target = PyGraphicDelta;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyGraphicDelta {
                graphic_id: self.graphic_id,
                padding_1: self.padding_1,
                sprite_ptr: self.sprite_ptr,
                offset_x: self.offset_x,
                offset_y: self.offset_y,
                display_angle: self.display_angle,
                padding_2: self.padding_2,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "GraphicAngleSound", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyGraphicAngleSound {
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

    impl<'py> IntoPyObject<'py> for GraphicAngleSound {
        type Target = PyGraphicAngleSound;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyGraphicAngleSound {
                frame_num: self.frame_num,
                sound_id: self.sound_id,
                wwise_sound_id: self.wwise_sound_id,
                frame_num_2: self.frame_num_2,
                sound_id_2: self.sound_id_2,
                wwise_sound_id_2: self.wwise_sound_id_2,
                frame_num_3: self.frame_num_3,
                sound_id_3: self.sound_id_3,
                wwise_sound_id_3: self.wwise_sound_id_3,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Graphic", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyGraphic {
        name: Py<PyString>,
        file_name: Py<PyString>,
        particle_effect_name: Py<PyString>,
        slp: i32,
        is_loaded: u8,
        old_color_flag: u8,
        layer: u8,
        player_color: i16,
        transparent_selection: u8,
        coordinates: (i16, i16, i16, i16),
        sound_id: i16,
        wwise_sound_id: i32,
        angle_sounds_used: u8,
        frame_count: i16,
        angle_count: i16,
        speed_multiplier: f32,
        frame_duration: f32,
        replay_delay: f32,
        sequence_type: u8,
        id: i16,
        mirroring_mode: u8,
        editor_flag: u8,
        deltas: Py<PyList>,
        angle_sounds: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for Graphic {
        type Target = PyGraphic;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyGraphic {
                name: self.name.into_pyobject(py)?.unbind(),
                file_name: self.file_name.into_pyobject(py)?.unbind(),
                particle_effect_name: self.particle_effect_name.into_pyobject(py)?.unbind(),
                slp: self.slp,
                is_loaded: self.is_loaded,
                old_color_flag: self.old_color_flag,
                layer: self.layer,
                player_color: self.player_color,
                transparent_selection: self.transparent_selection,
                coordinates: self.coordinates,
                sound_id: self.sound_id,
                wwise_sound_id: self.wwise_sound_id,
                angle_sounds_used: self.angle_sounds_used,
                frame_count: self.frame_count,
                angle_count: self.angle_count,
                speed_multiplier: self.speed_multiplier,
                frame_duration: self.frame_duration,
                replay_delay: self.replay_delay,
                sequence_type: self.sequence_type,
                id: self.id,
                mirroring_mode: self.mirroring_mode,
                editor_flag: self.editor_flag,
                deltas: self.deltas.into_pyobject(py)?.downcast_into()?.unbind(),
                angle_sounds: self
                    .angle_sounds
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

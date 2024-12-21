use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::common::DebugString;
use crate::common::TERRAIN_COUNT;
use crate::common::TERRAIN_UNITS_SIZE;
use crate::common::TILE_TYPE_COUNT;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct FrameData {
    frame_count: i16,
    angle_count: i16,
    shape_id: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct Terrain {
    enabled: u8,
    random: u8,
    is_water: u8,
    hide_in_editor: u8,
    string_id: i32,
    name: DebugString,
    name_2: DebugString,
    slp: i32,
    shape_ptr: i32,
    sound_id: i32,
    wwise_sound_id: u32,
    wwise_sound_stop_id: u32,
    blend_priority: i32,
    blend_type: i32,
    overlay_mask_name: DebugString,
    colors: (u8, u8, u8),
    cliff_colors: (u8, u8),
    passable_terrain: u8,
    impassable_terrain: u8,
    is_animated: u8,
    animation_frames: i16,
    pause_frames: i16,
    interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: u8,
    drawn: u8,

    #[br(count = TILE_TYPE_COUNT)]
    frame_data: Vec<FrameData>,
    terrain_to_draw: i16,
    terrain_dimensions: (i16, i16),

    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_masked_density: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_id: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_density: Vec<i16>,
    #[br(count = TERRAIN_UNITS_SIZE)]
    terrain_unit_centering: Vec<u8>,

    number_of_terrain_units_used: i16,
    phantom: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct TileSize {
    width: i16,
    height: i16,
    delta_y: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub struct TerrainBlock {
    virtual_function_ptr: u32,
    map_pointer: u32,
    map_width: i32,
    map_height: i32,
    world_width: i32,
    world_height: i32,

    #[br(count = TILE_TYPE_COUNT)]
    tile_sizes: Vec<TileSize>,
    padding_ts: i16,
    #[br(count = TERRAIN_COUNT)]
    terrains: Vec<Terrain>,
    map_min_x: f32,
    map_min_y: f32,
    map_max_x: f32,
    map_max_y: f32,
    map_max_x_plus_1: f32,
    map_max_y_plus_1: f32,
    terrains_used_2: i16,
    borders_used: i16,
    max_terrain: i16,
    tile_width: i16,
    tile_height: i16,
    tile_half_height: i16,
    tile_half_width: i16,
    elev_height: i16,
    cur_row: i16,
    cur_col: i16,
    block_beg_row: i16,
    block_end_row: i16,
    block_beg_col: i16,
    block_end_col: i16,

    search_map_ptr: u32,
    search_map_rows_ptr: u32,
    any_frame_change: u8,
    map_visible_flag: u8,
    fog_flag: u8,
}

#[cfg(feature = "pyo3")]
pub mod python {
    use super::FrameData;
    use super::Terrain;
    use super::TerrainBlock;
    use super::TileSize;

    use pyo3::prelude::*;
    use pyo3::types::PyList;
    use pyo3::types::PyString;
    use pyo3::types::PyTuple;

    #[pyclass(name = "FrameData", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyFrameData {
        frame_count: i16,
        angle_count: i16,
        shape_id: i16,
    }
    impl<'py> IntoPyObject<'py> for FrameData {
        type Target = PyFrameData;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyFrameData {
                frame_count: self.frame_count,
                angle_count: self.angle_count,
                shape_id: self.shape_id,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Terrain", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTerrain {
        enabled: u8,
        random: u8,
        is_water: u8,
        hide_in_editor: u8,
        string_id: i32,
        name: Py<PyString>,
        name_2: Py<PyString>,
        slp: i32,
        shape_ptr: i32,
        sound_id: i32,
        wwise_sound_id: u32,
        wwise_sound_stop_id: u32,
        blend_priority: i32,
        blend_type: i32,
        overlay_mask_name: Py<PyString>,
        colors: (u8, u8, u8),
        cliff_colors: (u8, u8),
        passable_terrain: u8,
        impassable_terrain: u8,
        is_animated: u8,
        animation_frames: i16,
        pause_frames: i16,
        interval: f32,
        pause_between_loops: f32,
        frame: i16,
        draw_frame: i16,
        animate_last: f32,
        frame_changed: u8,
        drawn: u8,
        frame_data: Py<PyList>,
        terrain_to_draw: i16,
        terrain_dimensions: Py<PyTuple>,
        terrain_unit_masked_density: Py<PyList>,
        terrain_unit_id: Py<PyList>,
        terrain_unit_density: Py<PyList>,
        terrain_unit_centering: Py<PyList>,
        number_of_terrain_units_used: i16,
        phantom: i16,
    }
    impl<'py> IntoPyObject<'py> for Terrain {
        type Target = PyTerrain;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTerrain {
                enabled: self.enabled,
                random: self.random,
                is_water: self.is_water,
                hide_in_editor: self.hide_in_editor,
                string_id: self.string_id,
                name: self.name.into_pyobject(py)?.unbind(),
                name_2: self.name_2.into_pyobject(py)?.unbind(),
                slp: self.slp,
                shape_ptr: self.shape_ptr,
                sound_id: self.sound_id,
                wwise_sound_id: self.wwise_sound_id,
                wwise_sound_stop_id: self.wwise_sound_stop_id,
                blend_priority: self.blend_priority,
                blend_type: self.blend_type,
                overlay_mask_name: self.overlay_mask_name.into_pyobject(py)?.unbind(),
                colors: self.colors,
                cliff_colors: self.cliff_colors,
                passable_terrain: self.passable_terrain,
                impassable_terrain: self.impassable_terrain,
                is_animated: self.is_animated,
                animation_frames: self.animation_frames,
                pause_frames: self.pause_frames,
                interval: self.interval,
                pause_between_loops: self.pause_between_loops,
                frame: self.frame,
                draw_frame: self.draw_frame,
                animate_last: self.animate_last,
                frame_changed: self.frame_changed,
                drawn: self.drawn,
                frame_data: self.frame_data.into_pyobject(py)?.downcast_into()?.unbind(),
                terrain_to_draw: self.terrain_to_draw,
                terrain_dimensions: self.terrain_dimensions.into_pyobject(py)?.unbind(),
                terrain_unit_masked_density: self
                    .terrain_unit_masked_density
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                terrain_unit_id: self
                    .terrain_unit_id
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                terrain_unit_density: self
                    .terrain_unit_density
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                terrain_unit_centering: self
                    .terrain_unit_centering
                    .into_iter()
                    .map(|x| x as u16)
                    .collect::<Vec<u16>>() // Needed to avoid Vec<u8> -> bytes conversion
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                number_of_terrain_units_used: self.number_of_terrain_units_used,
                phantom: self.phantom,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "TileSize", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTileSize {
        width: i16,
        height: i16,
        delta_y: i16,
    }
    impl<'py> IntoPyObject<'py> for TileSize {
        type Target = PyTileSize;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTileSize {
                width: self.width,
                height: self.height,
                delta_y: self.delta_y,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "TerrainBlock", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyTerrainBlock {
        virtual_function_ptr: u32,
        map_pointer: u32,
        map_width: i32,
        map_height: i32,
        world_width: i32,
        world_height: i32,
        tile_sizes: Py<PyList>,
        padding_ts: i16,
        terrains: Py<PyList>,
        map_min_x: f32,
        map_min_y: f32,
        map_max_x: f32,
        map_max_y: f32,
        map_max_x_plus_1: f32,
        map_max_y_plus_1: f32,
        terrains_used_2: i16,
        borders_used: i16,
        max_terrain: i16,
        tile_width: i16,
        tile_height: i16,
        tile_half_height: i16,
        tile_half_width: i16,
        elev_height: i16,
        cur_row: i16,
        cur_col: i16,
        block_beg_row: i16,
        block_end_row: i16,
        block_beg_col: i16,
        block_end_col: i16,
        search_map_ptr: u32,
        search_map_rows_ptr: u32,
        any_frame_change: u8,
        map_visible_flag: u8,
        fog_flag: u8,
    }
    impl<'py> IntoPyObject<'py> for TerrainBlock {
        type Target = PyTerrainBlock;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyTerrainBlock {
                virtual_function_ptr: self.virtual_function_ptr,
                map_pointer: self.map_pointer,
                map_width: self.map_width,
                map_height: self.map_height,
                world_width: self.world_width,
                world_height: self.world_height,
                tile_sizes: self.tile_sizes.into_pyobject(py)?.downcast_into()?.unbind(),
                padding_ts: self.padding_ts,
                terrains: self.terrains.into_pyobject(py)?.downcast_into()?.unbind(),
                map_min_x: self.map_min_x,
                map_min_y: self.map_min_y,
                map_max_x: self.map_max_x,
                map_max_y: self.map_max_y,
                map_max_x_plus_1: self.map_max_x_plus_1,
                map_max_y_plus_1: self.map_max_y_plus_1,
                terrains_used_2: self.terrains_used_2,
                borders_used: self.borders_used,
                max_terrain: self.max_terrain,
                tile_width: self.tile_width,
                tile_height: self.tile_height,
                tile_half_height: self.tile_half_height,
                tile_half_width: self.tile_half_width,
                elev_height: self.elev_height,
                cur_row: self.cur_row,
                cur_col: self.cur_col,
                block_beg_row: self.block_beg_row,
                block_end_row: self.block_end_row,
                block_beg_col: self.block_beg_col,
                block_end_col: self.block_end_col,
                search_map_ptr: self.search_map_ptr,
                search_map_rows_ptr: self.search_map_rows_ptr,
                any_frame_change: self.any_frame_change,
                map_visible_flag: self.map_visible_flag,
                fog_flag: self.fog_flag,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

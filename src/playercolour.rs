use binrw::binrw;

#[binrw]
struct PlayerColour {
    id: i32,
    player_color_base: i32,
    unit_outline_color: i32,
    unit_selection_color_1: i32,
    unit_selection_color_2: i32,
    minimap_color: i32,
    minimap_color_2: i32,
    minimap_color_3: i32,
    statistics_text: i32,
}
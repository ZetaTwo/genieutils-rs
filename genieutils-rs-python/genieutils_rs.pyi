from typing import List, Optional, Tuple

class Task:
    task_type: int
    id: int
    is_default: int
    action_type: int
    class_id: int
    unit_id: int
    terrain_id: int
    resource_in: int
    resource_multiplier: int
    resource_out: int
    unused_resource: int
    work_value_1: float
    work_value_2: float
    work_range: float
    auto_search_targets: int
    search_wait_time: float
    enable_targeting: int
    combat_level_flag: int
    gather_type: int
    work_flag_2: int
    target_diplomacy: int
    carry_check: int
    pick_for_construction: int
    moving_graphic_id: int
    proceeding_graphic_id: int
    working_graphic_id: int
    carrying_graphic_id: int
    resource_gathering_sound_id: int
    resource_deposit_sound_id: int
    wwise_resource_gathering_sound_id: int
    wwise_resource_deposit_sound_id: int

class ResourceStorage:
    storage_type: int
    amount: float
    flag: int

class DamageGraphic:
    graphic_id: int
    damage_percent: int
    apply_mode: int

class DeadFish:
    walking_graphic: int
    running_graphic: int
    rotation_speed: float
    old_size_class: int
    tracking_unit: int
    tracking_unit_mode: int
    tracking_unit_density: float
    old_move_algorithm: int
    turn_radius: float
    turn_radius_speed: float
    max_yaw_per_second_moving: float
    stationary_yaw_revolution_time: float
    max_yaw_per_second_stationary: float
    min_collision_size_multiplier: float

class Bird:
    default_task_id: int
    search_radius: float
    work_rate: float
    drop_sites: List[int]
    task_swap_group: int
    attack_sound: int
    move_sound: int
    wwise_attack_sound_id: int
    wwise_move_sound_id: int
    run_pattern: int
    tasks: List[Task]

class AttackOrArmor:
    attribute_class: int
    amount: int

class Type50:
    base_armor: int
    attacks: List[AttackOrArmor]
    armours: List[AttackOrArmor]
    defense_terrain_bonus: int
    bonus_damage_resistance: float
    max_range: float
    blast_width: float
    reload_time: float
    projectile_unit_id: int
    accuracy_percent: int
    break_off_combat: int
    frame_delay: int
    graphic_displacement: Tuple[float, float, float]
    blast_attack_level: int
    min_range: float
    accuracy_dispersion: float
    attack_graphic: int
    displayed_melee_armour: int
    displayed_attack: int
    displayed_range: float
    displayed_reload_time: float
    blast_damage: float

class Projectile:
    projectile_type: int
    smart_mode: int
    hit_mode: int
    vanish_mode: int
    area_effect_specials: int
    projectile_arc: float

class ResourceCost:
    cost_type: int
    amount: int
    flag: int

class Creatable:
    resource_costs: Tuple[ResourceCost, ResourceCost, ResourceCost]
    train_time: int
    train_location_id: int
    button_id: int
    rear_attack_modifier: float
    flank_attack_modifier: float
    creatable_type: int
    hero_mode: int
    garrison_graphic: int
    spawning_graphic: int
    upgrade_graphic: int
    hero_glow_graphic: int
    max_charge: float
    recharge_rate: float
    charge_event: int
    charge_type: int
    min_conversion_time_mod: float
    max_conversion_time_mod: float
    conversion_chance_mod: float
    total_projectiles: float
    max_total_projectiles: int
    projectile_spawning_area: Tuple[float, float, float]
    secondary_projectile_unit: int
    special_graphic: int
    special_ability: int
    displayed_pierce_armor: int

class BuildingAnnex:
    unit_id: int
    misplacement_x: float
    misplacement_y: float

class Building:
    construction_graphic_id: int
    snow_graphic_id: int
    destruction_graphic_id: int
    destruction_rubble_graphic_id: int
    researching_graphic: int
    research_completed_graphic: int
    adjacent_mode: int
    graphics_angle: int
    disappears_when_built: int
    stack_unit_id: int
    foundation_terrain_id: int
    old_overlap_id: int
    tech_id: int
    can_burn: int
    annexes: Tuple[BuildingAnnex, BuildingAnnex, BuildingAnnex, BuildingAnnex]
    head_unit: int
    transform_unit: int
    transform_sound: int
    construction_sound: int
    wwise_transform_sound_id: int
    wwise_construction_sound_id: int
    garrison_type: int
    garrison_heal_rate: float
    garrison_repair_rate: float
    pile_unit: int
    looting_table: Tuple[int, int, int, int, int, int]

class Unit:
    unit_type: int
    id: int
    language_dll_name: int
    language_dll_creation: int
    unit_class: int
    standing_graphic: Tuple[int, int]
    dying_graphic: int
    undead_graphic: int
    undead_mode: int
    hit_points: int
    line_of_sight: float
    garrison_capacity: int
    collision_size_x: float
    collision_size_y: float
    collision_size_z: float
    train_sound: int
    damage_sound: int
    dead_unit_id: int
    blood_unit_id: int
    sort_number: int
    can_be_built_on: int
    icon_id: int
    hide_in_editor: int
    old_portrait_pict: int
    enabled: int
    disabled: int
    placement_side_terrain: Tuple[int, int]
    placement_terrain: Tuple[int, int]
    clearance_size: Tuple[float, float]
    hill_mode: int
    fog_visibility: int
    terrain_restriction: int
    fly_mode: int
    resource_capacity: int
    resource_decay: float
    blast_defense_level: int
    combat_level: int
    interation_mode: int
    minimap_mode: int
    interface_kind: int
    multiple_attribute_mode: float
    minimap_color: int
    language_dll_help: int
    language_dll_hotkey_text: int
    hot_key: int
    recyclable: int
    enable_auto_gather: int
    create_doppelganger_on_death: int
    resource_gather_group: int
    occlusion_mode: int
    obstruction_type: int
    obstruction_class: int
    unit_trait: int
    civilization: int
    nothing: int
    selection_effect: int
    editor_selection_colour: int
    outline_size_x: float
    outline_size_y: float
    outline_size_z: float
    scenario_triggers_1: int
    scenario_triggers_2: int
    resource_storages: Tuple[ResourceStorage, ResourceStorage, ResourceStorage]
    damage_graphics: List[DamageGraphic]
    selection_sound: int
    dying_sound: int
    wwise_train_sound_id: int
    wwise_damage_sound_id: int
    wwise_selection_sound_id: int
    wwise_dying_sound_id: int
    old_attack_reaction: int
    convert_terrain: int
    name: str
    copy_id: int
    base_id: int
    speed: Optional[float]
    dead_fish: Optional[DeadFish]
    bird: Optional[Bird]
    type_50: Optional[Type50]
    projectile: Optional[Projectile]
    creatable: Optional[Creatable]
    building: Optional[Building]

class Civ:
    player_type: int
    name: str
    tech_tree_id: int
    team_bonus_id: int
    resources: List[float]
    icon_set: int
    unit_pointers: List[int]
    units: List[Optional[Unit]]

# TODO: enum
class Version:
    pass

class TerrainPassGraphic:
    exit_tile_sprite_id: int
    enter_tile_sprite_id: int
    walk_tile_sprite_id: int
    walk_sprite_rate: int

class TerrainRestriction:
    passable_buildable_dmg_multiplier: List[float]
    terrain_pass_graphics: List[TerrainPassGraphic]

class PlayerColour:
    id: int
    player_color_base: int
    unit_outline_color: int
    unit_selection_color_1: int
    unit_selection_color_2: int
    minimap_color: int
    minimap_color_2: int
    minimap_color_3: int
    statistics_text: int

class SoundItem:
    filename: str
    resource_id: int
    probability: int
    civilization: int
    icon_set: int

class Sound:
    id: int
    play_delay: int
    cache_time: int
    total_probability: int
    items: List[SoundItem]

class GraphicDelta:
    graphic_id: int
    padding_1: int
    sprite_ptr: int
    offset_x: int
    offset_y: int
    display_angle: int
    padding_2: int

class GraphicAngleSound:
    frame_num: int
    sound_id: int
    wwise_sound_id: int
    frame_num_2: int
    sound_id_2: int
    wwise_sound_id_2: int
    frame_num_3: int
    sound_id_3: int
    wwise_sound_id_3: int

class Graphic:
    name: str
    file_name: str
    particle_effect_name: str
    slp: int
    is_loaded: int
    old_color_flag: int
    layer: int
    player_color: int
    transparent_selection: int
    coordinates: Tuple[int, int, int, int]
    sound_id: int
    wwise_sound_id: int
    angle_sounds_used: int
    frame_count: int
    speed_multiplier: float
    frame_duration: float
    replay_delay: float
    sequence_type: int
    id: int
    mirroring_mode: int
    editor_flag: int
    deltas: List[GraphicDelta]
    angle_sounds: List[GraphicAngleSound]

class FrameData:
    frame_count: int
    angle_count: int
    shape_id: int

class Terrain:
    enabled: int
    random: int
    is_water: int
    hide_in_editor: int
    string_id: int
    name: str
    name_2: str
    slp: int
    shape_ptr: int
    sound_id: int
    wwise_sound_id: int
    wwise_sound_stop_id: int
    blend_priority: int
    blend_type: int
    overlay_mask_name: str
    colors: Tuple[int, int, int]
    cliff_colors: Tuple[int, int]
    passable_terrain: int
    impassable_terrain: int
    is_animated: int
    animation_frames: int
    pause_frames: int
    interval: float
    pause_between_loops: float
    frame: int
    draw_frame: int
    animate_last: float
    frame_changed: int
    drawn: int
    frame_data: List[FrameData]
    terrain_to_draw: int
    terrain_dimensions: Tuple[int, int]
    terrain_unit_masked_density: List[int]
    terrain_unit_id: List[int]
    terrain_unit_density: List[int]
    terrain_unit_centering: List[int]
    number_of_terrain_units_used: int
    phantom: int

class TileSize:
    width: int
    height: int
    delta_y: int

class TerrainBlock:
    virtual_function_ptr: int
    map_pointer: int
    map_width: int
    map_height: int
    world_width: int
    world_height: int
    tile_sizes: List[TileSize]
    padding_ts: int
    terrains: List[Terrain]
    map_min_x: float
    map_min_y: float
    map_max_x: float
    map_max_y: float
    map_max_x_plus_1: float
    map_max_y_plus_1: float
    terrains_used_2: int
    borders_used: int
    max_terrain: int
    tile_width: int
    tile_height: int
    tile_half_height: int
    tile_half_width: int
    elev_height: int
    cur_row: int
    cur_col: int
    block_beg_row: int
    block_end_row: int
    block_beg_col: int
    block_end_col: int
    search_map_ptr: int
    search_map_rows_ptr: int
    any_frame_change: int
    map_visible_flag: int
    fog_flag: int

class MapUnit:
    unit: int
    host_terrain: int
    group_placing: int
    scale_flag: int
    padding_1: int
    objects_per_group: int
    fluctuation: int
    groups_per_player: int
    group_arena: int
    player_id: int
    set_place_for_all_players: int
    min_distance_to_players: int
    max_distance_to_players: int

class MapTerrain:
    proportion: int
    terrain: int
    clump_count: int
    edge_spacing: int
    placement_terrain: int
    clumpiness: int

class MapLand:
    land_id: int
    terrain: int
    land_spacing: int
    base_size: int
    zone: int
    placement_type: int
    base_x: int
    base_y: int
    land_proportion: int
    by_player_flag: int
    start_area_radius: int
    terrain_edge_fade: int
    clumpiness: int

class MapElevation:
    proportion: int
    terrain: int
    clump_count: int
    base_terrain: int
    base_elevation: int
    tile_spacing: int

class MapInfo:
    map_id: int
    border_south_west: int
    border_north_west: int
    border_north_east: int
    border_south_east: int
    border_usage: int
    water_shape: int
    base_terrain: int
    land_coverage: int
    unused_id: int
    map_lands: List[MapLand]
    map_terrains: List[MapTerrain]
    map_units: List[MapUnit]
    map_elevations: List[MapElevation]

class RandomMaps:
    random_maps_ptr: int
    map_info_1: List[MapInfo]
    map_info_2: List[MapInfo]

class EffectCommand:
    effect_type: int
    a: int
    b: int
    c: int
    d: float

class Effect:
    name: str
    effect_commands: List[EffectCommand]

class TaskList:
    task_list: List[Task]

class UnitHeaders:
    exists: int
    task_list: Optional[TaskList]

class ResearchResourceCost:
    cost_type: int
    amount: int
    flag: int

class Tech:
    required_techs: Tuple[int, int, int, int, int, int]
    resource_costs: Tuple[
        ResearchResourceCost, ResearchResourceCost, ResearchResourceCost
    ]
    required_tech_count: int
    civ: int
    full_tech_mode: int
    research_location: int
    language_dll_name: int
    language_dll_description: int
    research_time: int
    effect_id: int
    tech_type: int
    icon_id: int
    button_id: int
    language_dll_help: int
    language_dll_tech_tree: int
    hot_key: int
    name: str
    repeatable: int

class Common:
    slots_used: int
    unit_research: Tuple[int, int, int, int, int, int, int, int, int, int]
    mode: Tuple[int, int, int, int, int, int, int, int, int, int]

class TechTreeAge:
    id: int
    status: int
    buildings: List[int]
    units: List[int]
    techs: List[int]
    common: Common
    num_building_levels: int
    buildings_per_zone: Tuple[int, int, int, int, int, int, int, int, int, int]
    group_length_per_zone: Tuple[int, int, int, int, int, int, int, int, int, int]
    max_age_length: int
    line_mode: int

class BuildingConnection:
    id: int
    status: int
    buildings: List[int]
    units: List[int]
    techs: List[int]
    common: Common
    location_in_age: int
    units_techs_total: Tuple[int, int, int, int, int]
    units_techs_first: Tuple[int, int, int, int, int]
    line_mode: int
    enabling_research: int

class UnitConnection:
    id: int
    status: int
    upper_building: int
    common: Common
    vertical_line: int
    units: List[int]
    location_in_age: int
    required_research: int
    line_mode: int
    enabling_research: int

class ResearchConnection:
    id: int
    status: int
    upper_building: int
    buildings: List[int]
    units: List[int]
    techs: List[int]
    common: Common
    vertical_line: int
    location_in_age: int
    line_mode: int

class TechTree:
    total_unit_tech_groups: int
    tech_tree_ages: List[TechTreeAge]
    building_connections: List[BuildingConnection]
    unit_connections: List[UnitConnection]
    research_connections: List[ResearchConnection]

class DatFile:
    version: Version
    terrains_used_1: int
    float_ptr_terrain_tables: List[int]
    terrain_pass_graphic_pointers: List[int]
    terrain_restrictions: List[TerrainRestriction]
    player_colours: List[PlayerColour]
    sounds: List[Sound]
    graphics_size: int
    graphic_pointers: List[int]
    graphics: List[Optional[Graphic]]
    terrain_block: TerrainBlock
    random_maps: RandomMaps
    effects: List[Effect]
    unit_headers: List[UnitHeaders]
    civs: List[Civ]
    techs: List[Tech]
    time_slice: int
    unit_kill_rate: int
    unit_kill_total: int
    unit_hit_point_rate: int
    unit_hit_point_total: int
    razing_kill_rate: int
    razing_kill_total: int
    tech_tree: TechTree

    @staticmethod
    def parse_compressed(data: bytes) -> DatFile: ...
    @staticmethod
    def parse(data: bytes) -> DatFile: ...
    @staticmethod
    def decompress(data: bytes) -> bytes: ...
    def serialize() -> bytes: ...
    def pack() -> bytes: ...

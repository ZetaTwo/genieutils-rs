use binrw::binrw;

use crate::common::DebugString;
use crate::common::UnitType;
use crate::task::Task;
use crate::versions::Version;

#[binrw]
struct ResourceStorage {
    r#type: i16,
    amount: f32,
    flag: u8,
}

#[binrw]
struct DamageGraphic {
    graphic_id: i16,
    damage_percent: i16,
    apply_mode: u8,
}

#[binrw]
struct DeadFish {
    walking_graphic: i16,
    running_graphic: i16,
    rotation_speed: f32,
    old_size_class: u8,
    tracking_unit: i16,
    tracking_unit_mode: u8,
    tracking_unit_density: f32,
    old_move_algorithm: u8,
    turn_radius: f32,
    turn_radius_speed: f32,
    max_yaw_per_second_moving: f32,
    stationary_yaw_revolution_time: f32,
    max_yaw_per_second_stationary: f32,
    min_collision_size_multiplier: f32,
}

#[binrw]
#[br(import(version: Version))]
#[bw(import(version: Version))]
struct Bird {
    default_task_id: i16,
    search_radius: f32,
    work_rate: f32,

    #[br(temp)]
    #[br(if(version > Version::Ver77, 3))]
    #[bw(if(version > Version::Ver77))]
    #[bw(try_calc = drop_sites.len().try_into())]
    drop_sites_size: i16,

    #[br(count = drop_sites_size)]
    drop_sites: Vec<i16>,

    task_swap_group: u8,
    attack_sound: i16,
    move_sound: i16,
    wwise_attack_sound_id: i32,
    wwise_move_sound_id: i32,
    run_pattern: u8,

    #[br(temp)]
    #[bw(try_calc = tasks.len().try_into())]
    tasks_size: i16,

    #[br(count = tasks_size)]
    tasks: Vec<Task>,
}

#[binrw]
struct AttackOrArmor {
    r#class: i16,
    amount: i16,
}

#[binrw]
struct Type50 {
    base_armor: i16,

    #[br(temp)]
    #[bw(try_calc = attacks.len().try_into())]
    attack_count: i16,

    #[br(count = attack_count)]
    attacks: Vec<AttackOrArmor>,

    #[br(temp)]
    #[bw(try_calc = armours.len().try_into())]
    armour_count: i16,

    #[br(count = armour_count)]
    armours: Vec<AttackOrArmor>,

    defense_terrain_bonus: i16,
    bonus_damage_resistance: f32,
    max_range: f32,
    blast_width: f32,
    reload_time: f32,
    projectile_unit_id: i16,
    accuracy_percent: i16,
    break_off_combat: u8,
    frame_delay: i16,
    graphic_displacement: (f32, f32, f32),
    blast_attack_level: u8,
    min_range: f32,
    accuracy_dispersion: f32,
    attack_graphic: i16,
    displayed_melee_armour: i16,
    displayed_attack: i16,
    displayed_range: f32,
    displayed_reload_time: f32,
    blast_damage: f32,
}

#[binrw]
struct Projectile {
    projectile_type: u8,
    smart_mode: u8,
    hit_mode: u8,
    vanish_mode: u8,
    area_effect_specials: u8,
    projectile_arc: f32,
}
#[binrw]
struct ResourceCost {
    r#type: i16,
    amount: i16,
    flag: i16,
}

#[binrw]
struct Creatable {
    resource_costs: (ResourceCost, ResourceCost, ResourceCost),
    train_time: i16,
    train_location_id: i16,
    button_id: u8,
    rear_attack_modifier: f32,
    flank_attack_modifier: f32,
    creatable_type: u8,
    hero_mode: u8,
    garrison_graphic: i32,
    spawning_graphic: i16,
    upgrade_graphic: i16,
    hero_glow_graphic: i16,
    max_charge: f32,
    recharge_rate: f32,
    charge_event: i16,
    charge_type: i16,
    min_conversion_time_mod: f32,
    max_conversion_time_mod: f32,
    conversion_chance_mod: f32,
    total_projectiles: f32,
    max_total_projectiles: u8,
    projectile_spawning_area: (f32, f32, f32),
    secondary_projectile_unit: i32,
    special_graphic: i32,
    special_ability: u8,
    displayed_pierce_armor: i16,
}

#[binrw]
struct BuildingAnnex {
    unit_id: i16,
    misplacement_x: f32,
    misplacement_y: f32,
}

#[binrw]
struct Building {
    construction_graphic_id: i16,
    snow_graphic_id: i16,
    destruction_graphic_id: i16,
    destruction_rubble_graphic_id: i16,
    researching_graphic: i16,
    research_completed_graphic: i16,
    adjacent_mode: u8,
    graphics_angle: i16,
    disappears_when_built: u8,
    stack_unit_id: i16,
    foundation_terrain_id: i16,
    old_overlap_id: i16,
    tech_id: i16,
    can_burn: u8,
    annexes: (BuildingAnnex, BuildingAnnex, BuildingAnnex, BuildingAnnex),
    head_unit: i16,
    transform_unit: i16,
    transform_sound: i16,
    construction_sound: i16,
    wwise_transform_sound_id: i32,
    wwise_construction_sound_id: i32,
    garrison_type: u8,
    garrison_heal_rate: f32,
    garrison_repair_rate: f32,
    pile_unit: i16,
    looting_table: (u8, u8, u8, u8, u8, u8),
}

#[binrw]
#[br(import(version: Version))]
#[bw(import(version: Version))]
pub struct Unit {
    r#type: u8,
    id: i16,
    language_dll_name: i32,
    language_dll_creation: i32,
    r#class: i16,
    standing_graphic: (i16, i16),
    dying_graphic: i16,
    undead_graphic: i16,
    undead_mode: u8,
    hit_points: i16,
    line_of_sight: f32,
    garrison_capacity: u8,
    collision_size_x: f32,
    collision_size_y: f32,
    collision_size_z: f32,
    train_sound: i16,
    damage_sound: i16,
    dead_unit_id: i16,
    blood_unit_id: i16,
    sort_number: u8,
    can_be_built_on: u8,
    icon_id: i16,
    hide_in_editor: u8,
    old_portrait_pict: i16,
    enabled: u8,
    disabled: u8,
    placement_side_terrain: (i16, i16),
    placement_terrain: (i16, i16),
    clearance_size: (f32, f32),
    hill_mode: u8,
    fog_visibility: u8,
    terrain_restriction: i16,
    fly_mode: u8,
    resource_capacity: i16,
    resource_decay: f32,
    blast_defense_level: u8,
    combat_level: u8,
    interation_mode: u8,
    minimap_mode: u8,
    interface_kind: u8,
    multiple_attribute_mode: f32,
    minimap_color: u8,
    language_dll_help: i32,
    language_dll_hotkey_text: i32,
    hot_key: i32,
    recyclable: u8,
    enable_auto_gather: u8,
    create_doppelganger_on_death: u8,
    resource_gather_group: u8,
    occlusion_mode: u8,
    obstruction_type: u8,
    obstruction_class: u8,
    r#trait: u8,
    civilization: u8,
    nothing: i16,
    selection_effect: u8,
    editor_selection_colour: u8,
    outline_size_x: f32,
    outline_size_y: f32,
    outline_size_z: f32,
    scenario_triggers_1: i32,
    scenario_triggers_2: i32,
    resource_storages: (ResourceStorage, ResourceStorage, ResourceStorage),

    #[br(temp)]
    #[bw(try_calc = damage_graphics.len().try_into())]
    damage_graphic_size: u8,

    #[br(count = damage_graphic_size)]
    damage_graphics: Vec<DamageGraphic>,

    selection_sound: i16,
    dying_sound: i16,
    wwise_train_sound_id: i32,
    wwise_damage_sound_id: i32,
    wwise_selection_sound_id: i32,
    wwise_dying_sound_id: i32,
    old_attack_reaction: u8,
    convert_terrain: u8,
    name: DebugString,
    copy_id: i16,
    base_id: i16,

    #[br(if(r#type >= UnitType::Flag))]
    #[bw(if(*r#type >= UnitType::Flag))]
    speed: Option<f32>,

    #[br(if(r#type >= UnitType::DeadFish))]
    #[bw(if(*r#type >= UnitType::DeadFish))]
    dead_fish: Option<DeadFish>,

    #[br(if(r#type >= UnitType::Bird))]
    #[bw(if(*r#type >= UnitType::Bird))]
    #[bw(args(version))]
    #[br(args(version))]
    bird: Option<Bird>,

    #[br(if(r#type >= UnitType::Combatant))]
    #[bw(if(*r#type >= UnitType::Combatant))]
    type_50: Option<Type50>,

    #[br(if(r#type == UnitType::Projectile))]
    #[bw(if(*r#type == UnitType::Projectile))]
    projectile: Option<Projectile>,

    #[br(if(r#type >= UnitType::Creatable))]
    #[bw(if(*r#type >= UnitType::Creatable))]
    creatable: Option<Creatable>,

    #[br(if(r#type == UnitType::Building))]
    #[bw(if(*r#type == UnitType::Building))]
    building: Option<Building>,
}

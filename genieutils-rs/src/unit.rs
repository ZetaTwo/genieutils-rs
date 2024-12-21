use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use crate::common::DebugString;
use crate::common::UnitType;
use crate::task::Task;
use crate::versions::Version;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct ResourceStorage {
    r#type: i16,
    amount: f32,
    flag: u8,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct DamageGraphic {
    graphic_id: i16,
    damage_percent: i16,
    apply_mode: u8,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct AttackOrArmor {
    r#class: i16,
    amount: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct Projectile {
    projectile_type: u8,
    smart_mode: u8,
    hit_mode: u8,
    vanish_mode: u8,
    area_effect_specials: u8,
    projectile_arc: f32,
}
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct ResourceCost {
    r#type: i16,
    amount: i16,
    flag: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
struct BuildingAnnex {
    unit_id: i16,
    misplacement_x: f32,
    misplacement_y: f32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
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

#[cfg(feature = "pyo3")]
mod python {
    use super::AttackOrArmor;
    use super::Bird;
    use super::Building;
    use super::BuildingAnnex;
    use super::Creatable;
    use super::DamageGraphic;
    use super::DeadFish;
    use super::Projectile;
    use super::ResourceCost;
    use super::ResourceStorage;
    use super::Type50;
    use super::Unit;

    use pyo3::prelude::*;
    use pyo3::types::PyAny;
    use pyo3::types::PyList;
    use pyo3::types::PyString;
    use pyo3::types::PyTuple;

    #[pyclass(name = "ResourceStorage", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]
    pub struct PyResourceStorage {
        r#type: i16,
        amount: f32,
        flag: u8,
    }

    impl<'py> IntoPyObject<'py> for ResourceStorage {
        type Target = PyResourceStorage;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyResourceStorage {
                r#type: self.r#type,
                amount: self.amount,
                flag: self.flag,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "DamageGraphic", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyDamageGraphic {
        graphic_id: i16,
        damage_percent: i16,
        apply_mode: u8,
    }

    impl<'py> IntoPyObject<'py> for DamageGraphic {
        type Target = PyDamageGraphic;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyDamageGraphic {
                graphic_id: self.graphic_id,
                damage_percent: self.damage_percent,
                apply_mode: self.apply_mode,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "DeadFish", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyDeadFish {
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

    impl<'py> IntoPyObject<'py> for DeadFish {
        type Target = PyDeadFish;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyDeadFish {
                walking_graphic: self.walking_graphic,
                running_graphic: self.running_graphic,
                rotation_speed: self.rotation_speed,
                old_size_class: self.old_size_class,
                tracking_unit: self.tracking_unit,
                tracking_unit_mode: self.tracking_unit_mode,
                tracking_unit_density: self.tracking_unit_density,
                old_move_algorithm: self.old_move_algorithm,
                turn_radius: self.turn_radius,
                turn_radius_speed: self.turn_radius_speed,
                max_yaw_per_second_moving: self.max_yaw_per_second_moving,
                stationary_yaw_revolution_time: self.stationary_yaw_revolution_time,
                max_yaw_per_second_stationary: self.max_yaw_per_second_stationary,
                min_collision_size_multiplier: self.min_collision_size_multiplier,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Bird", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyBird {
        default_task_id: i16,
        search_radius: f32,
        work_rate: f32,
        drop_sites: Py<PyList>,
        task_swap_group: u8,
        attack_sound: i16,
        move_sound: i16,
        wwise_attack_sound_id: i32,
        wwise_move_sound_id: i32,
        run_pattern: u8,
        tasks: Py<PyList>,
    }

    impl<'py> IntoPyObject<'py> for Bird {
        type Target = PyBird;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyBird {
                default_task_id: self.default_task_id,
                search_radius: self.search_radius,
                work_rate: self.work_rate,
                drop_sites: self.drop_sites.into_pyobject(py)?.downcast_into()?.unbind(),
                task_swap_group: self.task_swap_group,
                attack_sound: self.attack_sound,
                move_sound: self.move_sound,
                wwise_attack_sound_id: self.wwise_attack_sound_id,
                wwise_move_sound_id: self.wwise_move_sound_id,
                run_pattern: self.run_pattern,
                tasks: self.tasks.into_pyobject(py)?.downcast_into()?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "AttackOrArmor", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyAttackOrArmor {
        r#class: i16,
        amount: i16,
    }

    impl<'py> IntoPyObject<'py> for AttackOrArmor {
        type Target = PyAttackOrArmor;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyAttackOrArmor {
                r#class: self.r#class,
                amount: self.amount,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Type50", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyType50 {
        base_armor: i16,
        attacks: Py<PyList>,
        armours: Py<PyList>,
        defense_terrain_bonus: i16,
        bonus_damage_resistance: f32,
        max_range: f32,
        blast_width: f32,
        reload_time: f32,
        projectile_unit_id: i16,
        accuracy_percent: i16,
        break_off_combat: u8,
        frame_delay: i16,
        graphic_displacement: Py<PyTuple>,
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

    impl<'py> IntoPyObject<'py> for Type50 {
        type Target = PyType50;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyType50 {
                base_armor: self.base_armor,
                attacks: self.attacks.into_pyobject(py)?.downcast_into()?.unbind(),
                armours: self.armours.into_pyobject(py)?.downcast_into()?.unbind(),
                defense_terrain_bonus: self.defense_terrain_bonus,
                bonus_damage_resistance: self.bonus_damage_resistance,
                max_range: self.max_range,
                blast_width: self.blast_width,
                reload_time: self.reload_time,
                projectile_unit_id: self.projectile_unit_id,
                accuracy_percent: self.accuracy_percent,
                break_off_combat: self.break_off_combat,
                frame_delay: self.frame_delay,
                graphic_displacement: self.graphic_displacement.into_pyobject(py)?.unbind(),
                blast_attack_level: self.blast_attack_level,
                min_range: self.min_range,
                accuracy_dispersion: self.accuracy_dispersion,
                attack_graphic: self.attack_graphic,
                displayed_melee_armour: self.displayed_melee_armour,
                displayed_attack: self.displayed_attack,
                displayed_range: self.displayed_range,
                displayed_reload_time: self.displayed_reload_time,
                blast_damage: self.blast_damage,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Projectile", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyProjectile {
        projectile_type: u8,
        smart_mode: u8,
        hit_mode: u8,
        vanish_mode: u8,
        area_effect_specials: u8,
        projectile_arc: f32,
    }

    impl<'py> IntoPyObject<'py> for Projectile {
        type Target = PyProjectile;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyProjectile {
                projectile_type: self.projectile_type,
                smart_mode: self.smart_mode,
                hit_mode: self.hit_mode,
                vanish_mode: self.vanish_mode,
                area_effect_specials: self.area_effect_specials,
                projectile_arc: self.projectile_arc,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "ResourceCost", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyResourceCost {
        r#type: i16,
        amount: i16,
        flag: i16,
    }

    impl<'py> IntoPyObject<'py> for ResourceCost {
        type Target = PyResourceCost;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyResourceCost {
                r#type: self.r#type,
                amount: self.amount,
                flag: self.flag,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Creatable", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyCreatable {
        resource_costs: Py<PyTuple>,
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
        projectile_spawning_area: Py<PyTuple>,
        secondary_projectile_unit: i32,
        special_graphic: i32,
        special_ability: u8,
        displayed_pierce_armor: i16,
    }

    impl<'py> IntoPyObject<'py> for Creatable {
        type Target = PyCreatable;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyCreatable {
                resource_costs: self.resource_costs.into_pyobject(py)?.unbind(),
                train_time: self.train_time,
                train_location_id: self.train_location_id,
                button_id: self.button_id,
                rear_attack_modifier: self.rear_attack_modifier,
                flank_attack_modifier: self.flank_attack_modifier,
                creatable_type: self.creatable_type,
                hero_mode: self.hero_mode,
                garrison_graphic: self.garrison_graphic,
                spawning_graphic: self.spawning_graphic,
                upgrade_graphic: self.upgrade_graphic,
                hero_glow_graphic: self.hero_glow_graphic,
                max_charge: self.max_charge,
                recharge_rate: self.recharge_rate,
                charge_event: self.charge_event,
                charge_type: self.charge_type,
                min_conversion_time_mod: self.min_conversion_time_mod,
                max_conversion_time_mod: self.max_conversion_time_mod,
                conversion_chance_mod: self.conversion_chance_mod,
                total_projectiles: self.total_projectiles,
                max_total_projectiles: self.max_total_projectiles,
                projectile_spawning_area: self.projectile_spawning_area.into_pyobject(py)?.unbind(),
                secondary_projectile_unit: self.secondary_projectile_unit,
                special_graphic: self.special_graphic,
                special_ability: self.special_ability,
                displayed_pierce_armor: self.displayed_pierce_armor,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "BuildingAnnex", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyBuildingAnnex {
        unit_id: i16,
        misplacement_x: f32,
        misplacement_y: f32,
    }

    impl<'py> IntoPyObject<'py> for BuildingAnnex {
        type Target = PyBuildingAnnex;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyBuildingAnnex {
                unit_id: self.unit_id,
                misplacement_x: self.misplacement_x,
                misplacement_y: self.misplacement_y,
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Building", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyBuilding {
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
        annexes: Py<PyTuple>,
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
        looting_table: Py<PyTuple>,
    }

    impl<'py> IntoPyObject<'py> for Building {
        type Target = PyBuilding;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyBuilding {
                construction_graphic_id: self.construction_graphic_id,
                snow_graphic_id: self.snow_graphic_id,
                destruction_graphic_id: self.destruction_graphic_id,
                destruction_rubble_graphic_id: self.destruction_rubble_graphic_id,
                researching_graphic: self.researching_graphic,
                research_completed_graphic: self.research_completed_graphic,
                adjacent_mode: self.adjacent_mode,
                graphics_angle: self.graphics_angle,
                disappears_when_built: self.disappears_when_built,
                stack_unit_id: self.stack_unit_id,
                foundation_terrain_id: self.foundation_terrain_id,
                old_overlap_id: self.old_overlap_id,
                tech_id: self.tech_id,
                can_burn: self.can_burn,
                annexes: self.annexes.into_pyobject(py)?.unbind(),
                head_unit: self.head_unit,
                transform_unit: self.transform_unit,
                transform_sound: self.transform_sound,
                construction_sound: self.construction_sound,
                wwise_transform_sound_id: self.wwise_transform_sound_id,
                wwise_construction_sound_id: self.wwise_construction_sound_id,
                garrison_type: self.garrison_type,
                garrison_heal_rate: self.garrison_heal_rate,
                garrison_repair_rate: self.garrison_repair_rate,
                pile_unit: self.pile_unit,
                looting_table: self.looting_table.into_pyobject(py)?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }

    #[pyclass(name = "Unit", module = "genieutils_rspy")]
    #[pyo3(get_all, set_all)]

    pub struct PyUnit {
        r#type: u8,
        id: i16,
        language_dll_name: i32,
        language_dll_creation: i32,
        r#class: i16,
        standing_graphic: Py<PyTuple>,
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
        placement_side_terrain: Py<PyTuple>,
        placement_terrain: Py<PyTuple>,
        clearance_size: Py<PyTuple>,
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
        resource_storages: Py<PyTuple>,
        damage_graphics: Py<PyList>,
        selection_sound: i16,
        dying_sound: i16,
        wwise_train_sound_id: i32,
        wwise_damage_sound_id: i32,
        wwise_selection_sound_id: i32,
        wwise_dying_sound_id: i32,
        old_attack_reaction: u8,
        convert_terrain: u8,
        name: Py<PyString>,
        copy_id: i16,
        base_id: i16,
        speed: Option<f32>,
        dead_fish: Py<PyAny>,
        bird: Py<PyAny>,
        type_50: Py<PyAny>,
        projectile: Py<PyAny>,
        creatable: Py<PyAny>,
        building: Py<PyAny>,
    }

    impl<'py> IntoPyObject<'py> for Unit {
        type Target = PyUnit;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            let res = PyUnit {
                r#type: self.r#type,
                id: self.id,
                language_dll_name: self.language_dll_name,
                language_dll_creation: self.language_dll_creation,
                r#class: self.r#class,
                standing_graphic: self.standing_graphic.into_pyobject(py)?.unbind(),
                dying_graphic: self.dying_graphic,
                undead_graphic: self.undead_graphic,
                undead_mode: self.undead_mode,
                hit_points: self.hit_points,
                line_of_sight: self.line_of_sight,
                garrison_capacity: self.garrison_capacity,
                collision_size_x: self.collision_size_x,
                collision_size_y: self.collision_size_y,
                collision_size_z: self.collision_size_z,
                train_sound: self.train_sound,
                damage_sound: self.damage_sound,
                dead_unit_id: self.dead_unit_id,
                blood_unit_id: self.blood_unit_id,
                sort_number: self.sort_number,
                can_be_built_on: self.can_be_built_on,
                icon_id: self.icon_id,
                hide_in_editor: self.hide_in_editor,
                old_portrait_pict: self.old_portrait_pict,
                enabled: self.enabled,
                disabled: self.disabled,
                placement_side_terrain: self.placement_side_terrain.into_pyobject(py)?.unbind(),
                placement_terrain: self.placement_terrain.into_pyobject(py)?.unbind(),
                clearance_size: self.clearance_size.into_pyobject(py)?.unbind(),
                hill_mode: self.hill_mode,
                fog_visibility: self.fog_visibility,
                terrain_restriction: self.terrain_restriction,
                fly_mode: self.fly_mode,
                resource_capacity: self.resource_capacity,
                resource_decay: self.resource_decay,
                blast_defense_level: self.blast_defense_level,
                combat_level: self.combat_level,
                interation_mode: self.interation_mode,
                minimap_mode: self.minimap_mode,
                interface_kind: self.interface_kind,
                multiple_attribute_mode: self.multiple_attribute_mode,
                minimap_color: self.minimap_color,
                language_dll_help: self.language_dll_help,
                language_dll_hotkey_text: self.language_dll_hotkey_text,
                hot_key: self.hot_key,
                recyclable: self.recyclable,
                enable_auto_gather: self.enable_auto_gather,
                create_doppelganger_on_death: self.create_doppelganger_on_death,
                resource_gather_group: self.resource_gather_group,
                occlusion_mode: self.occlusion_mode,
                obstruction_type: self.obstruction_type,
                obstruction_class: self.obstruction_class,
                r#trait: self.r#trait,
                civilization: self.civilization,
                nothing: self.nothing,
                selection_effect: self.selection_effect,
                editor_selection_colour: self.editor_selection_colour,
                outline_size_x: self.outline_size_x,
                outline_size_y: self.outline_size_y,
                outline_size_z: self.outline_size_z,
                scenario_triggers_1: self.scenario_triggers_1,
                scenario_triggers_2: self.scenario_triggers_2,
                resource_storages: self.resource_storages.into_pyobject(py)?.unbind(),
                damage_graphics: self
                    .damage_graphics
                    .into_pyobject(py)?
                    .downcast_into()?
                    .unbind(),
                selection_sound: self.selection_sound,
                dying_sound: self.dying_sound,
                wwise_train_sound_id: self.wwise_train_sound_id,
                wwise_damage_sound_id: self.wwise_damage_sound_id,
                wwise_selection_sound_id: self.wwise_selection_sound_id,
                wwise_dying_sound_id: self.wwise_dying_sound_id,
                old_attack_reaction: self.old_attack_reaction,
                convert_terrain: self.convert_terrain,
                name: self.name.into_pyobject(py)?.unbind(),
                copy_id: self.copy_id,
                base_id: self.base_id,
                speed: self.speed,
                dead_fish: self.dead_fish.into_pyobject(py)?.unbind(),
                bird: self.bird.into_pyobject(py)?.unbind(),
                type_50: self.type_50.into_pyobject(py)?.unbind(),
                projectile: self.projectile.into_pyobject(py)?.unbind(),
                creatable: self.creatable.into_pyobject(py)?.unbind(),
                building: self.building.into_pyobject(py)?.unbind(),
            };

            Ok(Py::new(py, res)?.into_bound(py))
        }
    }
}

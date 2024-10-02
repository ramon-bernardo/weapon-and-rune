use bevy::prelude::*;
use rune::{Any, ContextError, Module};

#[derive(Component, Default, Debug)]
pub struct WeaponId {
    pub id: u32,
}

#[derive(Component, Default, Debug)]
pub struct Level(pub u32);

#[derive(Component, Default, Debug)]
pub struct MagicLevel(pub u32);

#[derive(Component, Default, Debug)]
pub struct Mana(pub u32);

#[derive(Component, Default, Debug)]
pub struct WandDamageRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Component, Default, Debug)]
pub struct DistanceBreakChance(pub u32);

#[derive(Component, Debug)]
pub struct WandWeapon;

#[derive(Component, Debug)]
pub struct MeleeWeapon;

#[derive(Component, Debug)]
pub struct DistanceWeapon;

// Creates a Rune module for managing weapons, registering methods that can be called from scripts.
pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::new();

    module.ty::<Weapon>()?;
    module.function_meta(Weapon::new_wand)?;
    module.function_meta(Weapon::new_distance)?;
    module.function_meta(Weapon::new_melee)?;
    module.function_meta(Weapon::level)?;
    module.function_meta(Weapon::magic_level)?;
    module.function_meta(Weapon::mana)?;
    module.function_meta(Weapon::damage_range)?;
    module.function_meta(Weapon::break_chance)?;

    Ok(module)
}

// Struct representing a Weapon with various optional attributes depending on type
#[derive(Any, Default)]
pub struct Weapon {
    id: u32,
    wand: Option<WandWeapon>,
    distance: Option<DistanceWeapon>,
    melee: Option<MeleeWeapon>,
    level: Option<Level>,
    magic_level: Option<MagicLevel>,
    mana: Option<Mana>,
    damage_range: Option<WandDamageRange>,
    break_chance: Option<DistanceBreakChance>,
}

impl Weapon {
    #[rune::function(path = Self::new_wand)]
    fn new_wand(id: u32) -> Self {
        Self {
            id,
            wand: Some(WandWeapon),
            ..Default::default()
        }
    }

    #[rune::function(path = Self::new_distance)]
    fn new_distance(id: u32) -> Self {
        Self {
            id,
            distance: Some(DistanceWeapon),
            ..Default::default()
        }
    }

    #[rune::function(path = Self::new_melee)]
    fn new_melee(id: u32) -> Self {
        Self {
            id,
            melee: Some(MeleeWeapon),
            ..Default::default()
        }
    }

    #[rune::function(instance)]
    fn level(mut self, value: u32) -> Self {
        self.level = Some(Level(value));
        self
    }

    #[rune::function(instance)]
    fn magic_level(mut self, value: u32) -> Self {
        self.magic_level = Some(MagicLevel(value));
        self
    }

    #[rune::function(instance)]
    fn mana(mut self, value: u32) -> Self {
        self.mana = Some(Mana(value));
        self
    }

    #[rune::function(instance)]
    fn damage_range(mut self, value_min: u32, value_max: u32) -> Self {
        self.damage_range = Some(WandDamageRange {
            min: value_min,
            max: value_max,
        });
        self
    }

    #[rune::function(instance)]
    fn break_chance(mut self, value: u32) -> Self {
        self.break_chance = Some(DistanceBreakChance(value));
        self
    }
}

pub trait WeaponCommand {
    fn spawn_weapon(&mut self, weapon: Weapon);
}

impl<'w, 's> WeaponCommand for Commands<'w, 's> {
    fn spawn_weapon(&mut self, weapon: Weapon) {
        self.spawn(WeaponId { id: weapon.id })
            .insert_if_some(weapon.wand)
            .insert_if_some(weapon.distance)
            .insert_if_some(weapon.melee)
            .insert_if_some(weapon.level)
            .insert_if_some(weapon.magic_level)
            .insert_if_some(weapon.mana)
            .insert_if_some(weapon.damage_range)
            .insert_if_some(weapon.break_chance);
    }
}

pub trait InsertIfSome {
    fn insert_if_some<T>(self, component: Option<T>) -> Self
    where
        Self: Sized,
        T: Component;
}

impl<'w, 's> InsertIfSome for EntityCommands<'w> {
    fn insert_if_some<T>(self, component: Option<T>) -> Self
    where
        T: Component,
    {
        if let Some(c) = component {
            self.insert(c)
        } else {
            self
        }
    }
}

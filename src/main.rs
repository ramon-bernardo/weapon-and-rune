use bevy::prelude::*;
use rune::{self, Sources};

use rune_context::RuneContext;
use weapons::{DistanceWeapon, MeleeWeapon, WandWeapon, WeaponCommand, WeaponId};

mod rune_context;
mod weapons;

fn main() {
    // Build the app with minimal plugins
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, startup.pipe(handle_error)) // Pipes errors to the handle_error system if any occur during startup
        .add_systems(Update, update)
        .run();
}

fn startup(mut commands: Commands) -> anyhow::Result<()> {
    let sources = create_sources()?;

    // Initializes the Rune context with the provided sources
    let context = RuneContext::new(sources)?;

    // Creates a rune virtual machine (VM)
    let mut vm = context.vm();

    // Steps
    // 1. Executes the "main" function in the script and processes the result
    // 2. Deserializes the output from the script into a list of weapons
    // 3. Spawns each weapon as an entity in the world
    let output = vm.execute(["main"], ())?.complete().into_result()?;
    let weapons = rune::from_value::<Vec<weapons::Weapon>>(output)?;
    for weapon in weapons {
        commands.spawn_weapon(weapon);
    }

    // Inserts the Rune context as a resource in the world
    commands.insert_resource(context);

    Ok(())
}

// Queries all entities with each component
fn update(
    wand_weapons: Query<(Entity, &WandWeapon)>,
    melee_weapons: Query<(Entity, &MeleeWeapon, &WeaponId)>,
    distance_weapons: Query<(Entity, &DistanceWeapon)>,
) {
    wand_weapons.iter().for_each(|(entity, weapon)| {
        println!("Entity: {:?}, {:?}", entity, weapon);
    });

    melee_weapons
        .iter()
        .for_each(|(entity, weapon, weapon_id)| {
            println!("Entity: {:?}, {:?}, {:?}", entity, weapon, weapon_id);
        });

    distance_weapons.iter().for_each(|(entity, weapon)| {
        println!("Entity: {:?}, {:?}", entity, weapon);
    });
}

fn handle_error(In(result): In<anyhow::Result<()>>) {
    if let Err(err) = result {
        println!("Error: {err:?}");
    }
}

fn create_sources() -> anyhow::Result<Sources> {
    Ok(rune::sources!(
        entry => {
            pub fn main() {
                // Build a wand with id 1, level 30, and damage range between 20 and 30
                let staff = Weapon::new_wand(1)
                    .level(30)
                    .damage_range(20, 30);

                // Build a sword with id 2 and level 10
                let sword = Weapon::new_melee(2)
                    .level(10);

                // Build a spear with id 3, level 30, and break chance of 4
                let spear = Weapon::new_distance(3)
                    .level(30)
                    .break_chance(4);

                [staff, sword, spear]
            }
        }
    ))
}

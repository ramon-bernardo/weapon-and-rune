## Weapon System for Bevy with Rune Integration

This project demonstrates a custom weapon system built using the [Bevy Game Engine](https://bevyengine.org) and [Rune Programming Language](https://rune-rs.github.io/). It provides the ability to spawn different types of weapons (Wand, Melee, and Distance) with various attributes such as level, mana, damage range, and break chance. The system integrates with ECS ([Entity Component System](https://pt.wikipedia.org/wiki/Entity-component-system)).

## Features
* **Weapon Types**: Create Wand, Melee and Distance weapons with unique characteristics.
* **Rune Scripting**: Modify weapons and their attributes through scripts using Rune.
* **Component-Based Design**: Each weapon type and attribute is represented as a Bevy components.
* **Optional Components**: Use a custom trait to only insert components when needed.
* **Extendable**: Easily extend the system to support more weapon types or attributes.

### Weapon Types
* **Wand Weapon**: Supports magic levels, mana usage, and a damage range.
* **Melee Weapon**: Basic weapon type with potential for future customization.
* **Distance Weapon**: Includes a break chance attribute, indicating how likely it is to break.

### Usage
* **Defining Weapon Components**: Weapon components such as WandWeapon, MeleeWeapon, and DistanceWeapon are used to define the type of weapon. Additional components like Level, Mana, and DamageRange provide custom attributes for each weapon.

* **Creating Weapons**: Weapons are created using the Weapon struct, with factory methods to create Wand, Melee, and Distance weapons.
```rust
let wand = Weapon::new_wand(1)
	.level(10)
	.magic_level(5)
	.mana(100);

let sword = Weapon::new_melee(2)
	.level(15);

let bow = Weapon::new_distance(3)
	.break_chance(20);
```

* **Spawning Weapons**: Use the spawn_weapon method to add the weapon to the world.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
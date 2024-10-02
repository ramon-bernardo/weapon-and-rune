# Weapon System for Bevy with Rune Integration

This project showcases a weapon system built with the [Bevy Game Engine](https://bevyengine.org) and the [Rune Programming Language](https://rune-rs.github.io/). It allows you to spawn various weapon types (Wand, Melee, Distance) with attributes like level, mana, damage range, and break chance. The system utilizes the ECS (Entity Component System) architecture.

## Features
- **Weapon Types**: Create unique Wand, Melee, and Distance weapons.
- **Rune Scripting**: Modify weapon attributes using Rune scripts.
- **Component-Based Design**: Weapon types and attributes are represented as Bevy components.
- **Optional Components**: Use a custom trait to insert components as needed.
- **Extendable**: Easily add new weapon types or attributes.

### Weapon Types
- **Wand**: Includes magic levels, mana usage, and damage range.
- **Melee**: Basic weapon type with potential for future customization.
- **Distance**: Features a break chance attribute.

### Usage
- **Defining Weapon Components**: Use components like `WandWeapon`, `MeleeWeapon`, and `DistanceWeapon` to define weapon types. Additional components (e.g., `Level`, `Mana`, `DamageRange`) provide custom attributes.
  
- **Creating Weapons**: Create weapons using the `Weapon` struct:
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

- **Spawning Weapons**: Add weapons to the world using the `spawn_weapon` method.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
mod button;
#[path = "singletons/global_state.rs"]
mod global_state;
mod hud;
mod mage;
mod main_scene;
#[path = "enums/player_char_enums/character_control_state_machine.rs"]
mod player_char_enums;
pub mod plugin;
mod spells;
mod wolf;

use godot::prelude::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}

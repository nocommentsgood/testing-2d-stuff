mod button;
mod hud;
mod mage;
mod main_scene;
mod player_camera;
#[path = "enums/player_char_enums/character_control_state_machine.rs"]
mod player_char_enums;
pub mod plugin;
mod singletons;
mod spells;
mod wolf;

use godot::prelude::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}

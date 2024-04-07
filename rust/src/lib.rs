mod button;
mod resources;
pub mod utilities;
pub mod traits {
    pub mod characters;
    pub mod damageable;
    pub mod damaging;
    pub mod delete_me;
    pub mod global;
    pub mod health;
    pub mod utils;
}
mod enums {
    pub mod player_char_enums;
}
mod hud;
mod mage;
mod main_scene;
mod player_camera;
pub mod plugin;
mod singletons;
mod spells;
mod wolf;

use godot::prelude::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}

use crate::enums::player_char_enums::playable_variables::PlayableCharVariables;
use godot::{engine::ICharacterBody2D, prelude::*};

pub trait Playable: ICharacterBody2D {
    fn request_character_variable(&mut self, variable: PlayableCharVariables);

    fn character_variable_request();
}

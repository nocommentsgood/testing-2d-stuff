use crate::enums::player_char_enums::playable_variables::PlayableCharVariables;
use godot::{engine::ICharacterBody2D, prelude::*};

pub trait Playable {
    fn move_to_target(&mut self);

    fn turn_based_move_to_target(&mut self, target: Vector2);

    fn set_state_to_turn_based(&mut self);

    fn get_mouse(&mut self) -> Vector2;
}

pub trait PlayablePointer {
    fn request_character_variable(&mut self, variable: PlayableCharVariables);
}

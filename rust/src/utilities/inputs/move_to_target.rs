use godot::prelude::*;

use crate::traits::{
    characters::playable_character::Playable, utils::input_handling::input_command::Command,
};

#[derive(GodotClass)]
#[class(init)]
pub struct MoveToTarget;

impl Command for MoveToTarget {
    fn execute(&mut self, actor: &mut dyn Playable) {
        actor.move_to_target();
    }
}

use godot::prelude::*;

use crate::traits::{
    characters::playable_character::Playable, utils::input_handling::input_command::Command,
};

#[derive(GodotClass)]
#[class(init)]
pub struct EnterTurnBasedCommand;

impl Command for EnterTurnBasedCommand {
    fn execute(&mut self, actor: &mut dyn Playable) {
        actor.set_state_to_turn_based();
    }
}
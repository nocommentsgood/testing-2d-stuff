use godot::obj::Gd;

use crate::traits::{
    characters::playable_character::Playable, utils::input_handling::input_command::Command,
};

struct EnterTurnBasedCommand;

impl Command for EnterTurnBasedCommand {
    fn execute(actor: &dyn Playable) {
        todo!()
    }
}

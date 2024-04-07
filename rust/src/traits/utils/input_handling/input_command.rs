use godot::prelude::*;

use crate::traits::characters::playable_character::Playable;

pub trait Command {
    fn execute(actor: &dyn Playable);
}

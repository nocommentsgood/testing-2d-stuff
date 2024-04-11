use godot::prelude::*;

use crate::traits::characters::playable_character::Playable;

pub trait Command {
    fn execute(&mut self, actor: &mut dyn Playable);

    fn get_mouse(actor: &mut dyn Playable) {
        actor
    }
}

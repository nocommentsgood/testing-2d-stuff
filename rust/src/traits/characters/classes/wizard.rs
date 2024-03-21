use godot::prelude::*;

pub trait Playable {
    fn request_do_skill(&mut self, toggled: bool, skill_index: i16);

    fn cancel_current_skill(&mut self);
}

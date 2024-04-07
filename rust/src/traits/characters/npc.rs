use godot::prelude::*;

pub trait Npc {
    fn is_hostile(&self) -> bool;

    fn player_entered_hostile_range(&mut self);
}

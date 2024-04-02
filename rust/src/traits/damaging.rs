use godot::prelude::*;

use super::{damageable::Damageable, health::Health};
pub trait Damaging {
    fn do_damage(&mut self, entity: &mut impl Damageable);
}

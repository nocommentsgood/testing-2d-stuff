use super::health::Health;

pub trait Damageable: Health {
    fn take_damage(&mut self, amount: u16);
    fn t_damage(entity: &dyn Health);
}

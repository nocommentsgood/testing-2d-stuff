pub trait Health {
    fn health(&self) -> u16;

    fn restore_health(&mut self, amount: u16);
}

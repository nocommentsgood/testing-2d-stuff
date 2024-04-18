use godot::builtin::Vector2;

pub trait PlayerControllable {
    fn move_to_target(&mut self, target: Vector2);

    fn turn_based_move_to_target(&mut self, target: Vector2);

    fn set_state_to_turn_based(&mut self);
}

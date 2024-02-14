use godot::{
    engine::{CharacterBody2D, ICharacterBody2D, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Mage {
    #[export]
    speed: real,
    screen_size: Vector2,
    #[var]
    target: Vector2,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Mage {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 200.0,
            screen_size: Vector2::ZERO,
            target: Vector2::ZERO,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("click".into()) {
            self.target = self.base().get_global_mouse_position();
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let velocity = self.base().get_position().direction_to(self.target) * self.speed;
        self.base_mut().set_velocity(velocity);

        if self.base().get_position().distance_to(self.target) > 10.0 {
            self.base_mut().move_and_slide();
        }
    }
}

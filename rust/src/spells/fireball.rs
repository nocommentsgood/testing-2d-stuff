use godot::{
    engine::{AnimatedSprite2D, CharacterBody2D, IAnimatedSprite2D, ICharacterBody2D, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct FireballSpell {
    speed: real,
    target: Vector2,
    #[export]
    base_damage: real,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for FireballSpell {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 16.0,
            target: Vector2::ZERO,
            base_damage: 32.0,
            base,
        }
    }

    fn ready(&mut self) {
        let mut anim = self.base_mut().get_node_as::<AnimatedSprite2D>("Fireball");
        anim.set_animation("moving".into());
        anim.play();
    }

    fn unhandled_input(&mut self, input: Gd<InputEvent>) {
        if input.is_action_pressed("click".into()) {
            let target = self.base().get_global_mouse_position();
            self.target = target;
            self.base_mut().look_at(target);
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let velocity = self.base().get_position().direction_to(self.target) * self.speed;

        self.base_mut().set_velocity(velocity);

        if self.base().get_position().distance_to(self.target) > 10.0 {
            self.base_mut().move_and_collide(velocity);
        }
    }
}

#[godot_api]
impl FireballSpell {}

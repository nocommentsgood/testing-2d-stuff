use std::ops::Deref;

use godot::{
    engine::{
        AnimatedSprite2D, CharacterBody2D, IAnimatedSprite2D, ICharacterBody2D, InputEvent,
        InputEventMouse,
    },
    obj::bounds::implement_godot_bounds,
    prelude::*,
};

use super::my_spell::Spell;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct FireballSpell {
    #[var]
    speed: real,
    #[var]
    target: Vector2,
    max_distance: real,
    has_travelled: bool,
    #[var]
    distance_to_travel: real,
    velocity: Vector2,
    #[export]
    base_damage: real,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for FireballSpell {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 500.0,
            target: Vector2::ZERO,
            max_distance: 1000.0,
            distance_to_travel: 0.0,
            has_travelled: false,
            velocity: Vector2::ZERO,
            base_damage: 32.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.target = self.base().get_position();
        self.base_mut().set_physics_process(false);
        let mut anim = self.base_mut().get_node_as::<AnimatedSprite2D>("Fireball");
        anim.set_animation("moving".into());
        anim.play();
    }

    fn unhandled_input(&mut self, input: Gd<InputEvent>) {
        if input.is_action_pressed("click".into()) {
            let target = self.base().get_global_mouse_position();
            self.target = target;
            let distance = self.base().get_position().distance_to(self.target);
            let mut viewport = self.base().get_viewport().unwrap();

            if distance > self.max_distance {
                godot_print!("CANNOT CAST THAT FAR");
            } else {
                self.base_mut().look_at(target);
                self.base_mut().set_process_unhandled_input(false);
                self.base_mut().set_physics_process(true);
                viewport.set_input_as_handled();
            }
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let target = self.target;
        let distance = self.base().get_position().distance_to(self.target);

        let position = self.base().get_global_position();
        let velocity = position.direction_to(target) * self.speed;

        let mut timer = self
            .base()
            .get_tree()
            .unwrap()
            .create_timer(distance as f64 / self.speed as f64)
            .unwrap();

        // there is prob a better way of connecting this...
        // this continuously tries to connect timer to callable every physics frame
        // works but sloppy
        timer.connect("timeout".into(), self.base().callable("animate_explosion"));

        self.base_mut().set_velocity(velocity);
        if distance > 10.0 && distance < self.max_distance {
            self.base_mut().move_and_slide();
        }
    }
}

impl Spell for FireballSpell {}

#[godot_api]
impl FireballSpell {
    #[func]
    fn on_explosion_anim_finished(&mut self) {
        self.base_mut().call_deferred("free".into(), &[]);
    }

    #[func]
    fn free(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn animate_explosion(&mut self) {
        self.base_mut().set_physics_process(false);
        let mut anim = self.base().get_node_as::<AnimatedSprite2D>("Fireball");
        anim.set_animation("explosion".into());
        anim.play();
        anim.connect(
            "animation_finished".into(),
            self.base().callable("on_explosion_anim_finished"),
        );
    }
}

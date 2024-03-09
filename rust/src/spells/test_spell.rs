use godot::{
    engine::{AnimatedSprite2D, Area2D, IArea2D, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct TestSpell {
    #[var]
    target: Vector2,
    #[var]
    speed: real,
    #[var]
    velocity: Vector2,
    max_distance: real,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for TestSpell {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            target: Vector2::ZERO,
            speed: 500.0,
            velocity: Vector2::ZERO,
            max_distance: 1000.0,
            base,
        }
    }

    fn ready(&mut self) {
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
                let mut timer = self
                    .base()
                    .get_tree()
                    .unwrap()
                    .create_timer((distance / self.speed) as f64)
                    .unwrap();

                self.base_mut().look_at(target);
                self.base_mut().set_process_unhandled_input(false);
                self.base_mut().set_physics_process(true);
                timer.connect("timeout".into(), self.base().callable("animate_explosion"));
                timer.connect(
                    "timeout".into(),
                    self.base().callable("get_colliding_bodies"),
                );
                viewport.set_input_as_handled();
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let position = self.base().get_position();
        let velocity = (self.target - position).normalized() * self.speed * delta as f32;
        let distance = position.distance_to(self.target);

        if distance > 10.0 {
            self.base_mut().set_position(position + velocity);
        }
    }
}

#[godot_api]
impl TestSpell {
    #[signal]
    fn spell_hit_players(bodies: Array<Gd<Node2D>>);

    #[func]
    fn get_colliding_bodies(&mut self) {
        let bodies = self.base().get_overlapping_bodies();
        godot_print!("bodies: {}", bodies);
    }

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

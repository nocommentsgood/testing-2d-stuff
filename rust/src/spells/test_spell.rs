use crate::{
    mage::Mage,
    traits::{damageable::Damageable, damaging::Damaging, health::Health},
    wolf::Wolf,
};
use godot::{
    engine::{
        AnimatedSprite2D, Area2D, IArea2D, InputEvent, InputEventMouseMotion, Line2D, RayCast2D,
    },
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
    #[export]
    damage: u16,
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
            damage: 20,
            max_distance: 1000.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_physics_process(false);
        self.base_mut().set_process(true);
        let mut anim = self.base_mut().get_node_as::<AnimatedSprite2D>("Fireball");
        anim.set_animation("moving".into());
        anim.play();
    }

    fn unhandled_input(&mut self, input: Gd<InputEvent>) {
        if input.is_action_pressed("click".into()) {
            let target = self.base().get_global_mouse_position();
            self.target = target;
            let distance = self.base().get_global_position().distance_to(self.target);
            let mut viewport = self.base().get_viewport().unwrap();

            if distance > self.max_distance {
                godot_print!("CANNOT CAST THAT FAR");
            } else {
                self.base_mut().set_process(false);

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
                timer.connect("timeout".into(), self.base().callable("prepare_damage"));
                viewport.set_input_as_handled();
            }
        }
    }

    fn process(&mut self, delta: f64) {
        godot_print!("process is running");
    }

    fn physics_process(&mut self, delta: f64) {
        let position = self.base().get_global_position();
        let velocity = (self.target - position).normalized() * self.speed * delta as f32;
        let distance = position.distance_to(self.target);

        if distance > 10.0 {
            self.base_mut().set_global_position(position + velocity);
        }
    }
}

#[godot_api]
impl TestSpell {
    #[signal]
    fn spell_hit_players(bodies: Array<Gd<Node2D>>);

    #[func]
    fn get_colliding_bodies(&mut self) -> Array<Gd<Node2D>> {
        self.base().get_overlapping_bodies()
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

    // dynamic dispatch is not supported but there might be a better way to do this
    // otherwise would have to try cast to every damageable entity in the game
    #[func]
    fn prepare_damage(&mut self) {
        let bodies = self.get_colliding_bodies();
        for body in bodies.iter_shared() {
            let body = body.clone().try_cast::<Wolf>();
            if let Ok(mut wolf) = body {
                wolf.take_damage(self.damage);
                godot_print!("wolf took damage");
            }
        }
    }
}

// not sure how to actually implement this yet
impl crate::traits::damaging::Damaging for Gd<TestSpell> {
    fn do_damage(&mut self, entity: &mut impl Damageable) {
        let amount = self.bind().get_damage();
        entity.take_damage(amount);
    }
}

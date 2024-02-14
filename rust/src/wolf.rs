use godot::{
    engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Wolf {
    speed: real,
    screen_size: Vector2,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Wolf {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 200.0,
            screen_size: Vector2::ZERO,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        let mut velocity = Vector2::new(0.0, 0.0);
        let input = Input::singleton();

        if input.is_action_pressed("left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("up".into()) {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "run_down_left";
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "run_down_right";
                // animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play_ex().name(animation.into()).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
    }
}

#[godot_api]
impl Wolf {}

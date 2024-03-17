use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
struct PlayerCamera {
    #[export]
    speed: real,
    screen_size: Vector2,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for PlayerCamera {
    fn init(base: Base<Camera2D>) -> Self {
        PlayerCamera {
            speed: 100.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = Vector2::new(0.0, 0.0);

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("camera_right".into()) {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("camera_left".into()) {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("camera_down".into()) {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("camera_up".into()) {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;
        }

        let change = velocity * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}

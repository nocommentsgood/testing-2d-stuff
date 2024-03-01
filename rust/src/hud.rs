use godot::{
    engine::{CanvasLayer, ICanvasLayer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
struct Hud {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self { base }
    }
}

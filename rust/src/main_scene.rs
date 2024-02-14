use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

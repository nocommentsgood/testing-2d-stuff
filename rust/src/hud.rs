use godot::{
    engine::{Button, CanvasLayer, GridContainer, ICanvasLayer, InputEvent, InputEventMouse},
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

    fn unhandled_input(&mut self, input: Gd<InputEvent>) {
        // let button_grid = self
        //     .base()
        //     .get_node_as::<GridContainer>("SpellContainer/GridContainer");
        // godot_print!("container: {}", button_grid.get_rect());
        // let t = input.cast::<InputEventMouse>();
        // if t.is_action_pressed("click".into()) {
        //     godot_print!("mouse_click: {}", t.get_position());
        // }

        // for b in button_grid.get_children().iter_shared() {
        //     let b = b.cast::<Button>();
        //     if b.is_pressed() {
        //         godot_print!("button pressed");
        //     }
        // }
    }
}

#[godot_api]
impl Hud {
    #[signal]
    fn spell_slot_one_pressed();
}

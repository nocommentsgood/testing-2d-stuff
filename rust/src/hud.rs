use godot::{
    engine::{Button, CanvasLayer, GridContainer, ICanvasLayer, InputEvent},
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

    // fn unhandled_input(&mut self, _input: Gd<InputEvent>) {
    //     let buttons = self
    //         .base()
    //         .get_node_as::<GridContainer>("SpellContainer/GridContainer")
    //         .get_children();
    //
    //     for button in buttons.iter_shared() {
    //         if button.cast::<Button>().is_pressed() {
    //             godot_print!("button pressed");
    //         }
    //     }
    // }

    // fn process(&mut self, _delta: f64) {
    //     let buttons = self
    //         .base()
    //         .get_node_as::<GridContainer>("SpellContainer/GridContainer")
    //         .get_children();
    //
    //     for button in buttons.iter_shared() {
    //         let button = button.cast::<Button>();
    //         if button.is_toggle_mode() {
    //             godot_print!("button pressed");
    //             button.set_deferred_thread_group
    //         }
    //     }
    // }
}

#[godot_api]
impl Hud {
    // #[signal]
    // fn spell_button_clicked(spell_index: i16);
    //
    // #[func]
    // fn on_spell_button_clicked(&mut self) {}
}

use godot::{
    engine::{global::MouseButton, InputEvent, InputEventMouse, InputEventMouseButton},
    prelude::*,
};

use crate::traits::utils::input_handling::input_command::Command;

use super::{enter_turn_based_command::EnterTurnBasedCommand, move_to_target::MoveToTarget};

#[derive(GodotClass)]
#[class(init)]
pub struct InputHandler;

#[godot_api]
impl InputHandler {
    pub fn handle_input(input: Gd<InputEvent>) -> Option<Box<dyn Command>> {
        if input.is_action_pressed("enter_turn_based".into()) {
            return Some(Box::new(EnterTurnBasedCommand));
        }
        // if input.is_action_pressed("click".into()) {
        //     let t = input
        //     godot_print!("got input of type: {}", input);
        //     return Some(Box::new(MoveToTarget {
        //         target: Vector2::ZERO,
        //     }));
        // }

        if let Ok(input) = input.try_cast::<InputEventMouseButton>() {
            if input.get_button_index() == MouseButton::LEFT {
                let t = input.get_position();
                godot_print!("got position: {}", t);
                return Some(Box::new(MoveToTarget));
            }
        }

        None
    }
}

use godot::engine::InputEvent;

use crate::traits::utils::input_handling::input_command::Command;

struct InputHandler;

impl InputHandler {
    pub fn handle_input(input: InputEvent) -> Box<dyn Command> {}
}

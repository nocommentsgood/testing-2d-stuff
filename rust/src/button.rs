use godot::{
    engine::{Button, IButton, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
struct SpellButton {
    #[export]
    spell_slot_number: i16,
    base: Base<Button>,
}

#[godot_api]
impl IButton for SpellButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            spell_slot_number: 0,
            base,
        }
    }

    fn gui_input(&mut self, input: Gd<InputEvent>) {
        if input.is_action_pressed("click".into()) {
            self.base_mut()
                .emit_signal("spell_button_clicked".into(), &[]);
        }
    }
}

#[godot_api]
impl SpellButton {
    #[signal]
    fn spell_button_clicked() {}
}

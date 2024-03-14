use godot::{
    engine::{Button, IButton},
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

    fn toggled(&mut self, toggled: bool) {
        let toggled = toggled.to_variant();
        let spell_index = self.get_spell_slot_number().to_variant();
        self.base_mut().accept_event();
        self.base_mut()
            .emit_signal("casting_spell".into(), &[toggled, spell_index]);
    }
}

#[godot_api]
impl SpellButton {
    #[signal]
    fn casting_spell(toggled: bool, spell_index: i16);
}

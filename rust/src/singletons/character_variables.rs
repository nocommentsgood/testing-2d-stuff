use std::collections::HashMap;

use godot::prelude::*;

use crate::enums::player_char_enums::skills::PlayerSkills;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerVariables {
    pub active_skills: Dictionary,
    #[export]
    path: NodePath,

    base: Base<Node>,
}

#[godot_api]
impl PlayerVariables {
    pub fn insert_active_skills(&mut self, index: i16, skill: PlayerSkills) {
        // TODO: Would like to make active_skills a #[var], look into how godot plays with rust
        // enums. Also how it plays with rust Hashmap. There is gdext dictionary type but this works for now
        self.active_skills.insert(index, skill);
    }
}

#[godot_api]
impl INode for PlayerVariables {
    fn init(base: Base<Node>) -> Self {
        Self {
            active_skills: Dictionary::new(),
            path: "".into(),
            base,
        }
    }

    fn ready(&mut self) {
        self.path = self.base().get_path();
        self.active_skills.insert(1, PlayerSkills::FIREBALL);
        self.active_skills.insert(2, PlayerSkills::TEST_SPELL);
    }
}

use std::collections::HashMap;

use godot::prelude::*;

use crate::enums::player_char_enums::skills::PlayerSkills;

// TODO: this class might be better as a resource
#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerVariables {
    pub active_skills: HashMap<i16, PlayerSkills>,
    pub health: i32,
    base: Base<Node>,
}

#[godot_api]
impl PlayerVariables {
    pub fn insert_active_skills(&mut self, index: i16, skill: PlayerSkills) {
        self.active_skills.insert(index, skill);
    }

    pub fn remove_active_skills(&mut self, index: i16) {
        self.active_skills.remove(&index);
    }
}

#[godot_api]
impl INode for PlayerVariables {
    fn init(base: Base<Node>) -> Self {
        Self {
            active_skills: HashMap::new(),
            health: 100,
            base,
        }
    }

    fn ready(&mut self) {
        self.active_skills.insert(1, PlayerSkills::FIREBALL);
        self.active_skills.insert(2, PlayerSkills::TEST_SPELL);
    }
}

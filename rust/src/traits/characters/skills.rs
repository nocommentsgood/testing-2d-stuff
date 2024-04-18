use godot::builtin::Dictionary;

use crate::enums::player_char_enums::skills::PlayerSkills;

use super::character_variables::PlayerVariables;

pub trait Skills: PlayerVariables {
    fn get_skills(&self) -> Dictionary {
        self.get_variable_resource().get_active_skills()
    }

    fn add_skill(&mut self);
}

use godot::prelude::*;

use crate::{
    resources::player_vars::PlayerVariableResource,
    traits::characters::{
        character_variables::PlayerVariables, playable_character::PlayerControllable,
        skills::Skills,
    },
};

/// Inject into a playable character to provide functions for using skills

#[derive(GodotClass)]
#[class(init)]
pub struct PlayerSkillComponent {
    player_vars: Gd<PlayerVariableResource>,
}

#[godot_api]
impl PlayerSkillComponent {
    pub fn cast_player_skill(&mut self, actor: Box<dyn PlayerVariables>) {
        let h = self.player_vars.bind().get_active_skills();
    }
}

impl Skills for PlayerSkillComponent {
    fn add_skill(&mut self) {}

    fn get_skills(&self) -> Dictionary {
        self.get_variable_resource().get_active_skills()
    }
}

impl PlayerVariables for PlayerSkillComponent {
    fn get_skills(&mut self) -> godot::prelude::Dictionary {
        self.player_vars.bind().get_active_skills()
    }

    fn get_variable_resource(&mut self) -> &PlayerVariableResource {
        &self.player_vars.bind()
    }
}

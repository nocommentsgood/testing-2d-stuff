use godot::prelude::*;

use crate::{
    resources::player_vars::PlayerVariableResource,
    traits::characters::{character_variables::PlayerVariables, playable_character::Playable},
};

#[derive(GodotClass)]
#[class(init)]
pub struct PlayerSkillComponent {
    player_vars: Box<Gd<PlayerVariableResource>>,
}

#[godot_api]
impl PlayerSkillComponent {
    pub fn cast_player_skill(&mut self, actor: Gd<Box<dyn Playable>>) {}
}

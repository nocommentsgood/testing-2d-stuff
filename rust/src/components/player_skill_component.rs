use std::{borrow::BorrowMut, ops::DerefMut};

use godot::{
    obj::{NewGd, WithBaseField},
    prelude::*,
};

use crate::{
    enums::player_char_enums::skills::PlayerSkills,
    resources::player_vars::PlayerVariableResource,
    traits::characters::{
        character_variables::PlayerVariables, playable_character::PlayerControllable,
        skills::Skills,
    },
};

/// Inject into a playable character to provide functions for using skills

#[derive(GodotClass)]
#[class(no_init)]
pub struct PlayerSkillComponent {
    player_vars: Gd<PlayerVariableResource>,
}

#[godot_api]
impl PlayerSkillComponent {
    pub fn new() -> Self {
        Self {
            player_vars: PlayerVariableResource::new_gd(),
        }
    }
    pub fn try_cast_player_skill<P>(&mut self, mut actor: Gd<P>, index: i16)
    where
        P: Inherits<Node>,
    {
        let skills = self.player_vars.bind().get_active_skills();
        let skill_variant = skills.get(index);
        let x = skill_variant.unwrap().to::<PlayerSkills>();
        let skill_instance = PlayerSkills::load_skill(&x).unwrap();
        actor.upcast_mut().add_child(skill_instance);
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

    fn get_variable_resource(&self) -> &PlayerVariableResource {
        todo!();
    }
}

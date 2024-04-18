use godot::prelude::*;

use crate::{
    enums::player_char_enums::skills::PlayerSkills, resources::player_vars::PlayerVariableResource,
};

pub trait PlayerVariables {
    fn get_skills(&mut self) -> Dictionary;

    fn get_variable_resource(&self) -> &PlayerVariableResource;
}

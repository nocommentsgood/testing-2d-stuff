use godot::prelude::*;

use crate::resources::player_vars::PlayerVariableResource;

#[derive(GodotClass)]
#[class(init)]
pub struct PlayerVariables {
    player_var_resource: Option<Gd<PlayerVariableResource>>,
}

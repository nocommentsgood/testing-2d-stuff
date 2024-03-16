use godot::prelude::*;

use crate::{enums::player_char_enums::skills::PlayerSkills, mage::Mage};

use super::{
    action_loader::{self, SkillLoader},
    character_variables::PlayerVariables,
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionManager {
    #[var]
    action_loader: Option<Gd<SkillLoader>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionManager {
    fn ready(&mut self) {
        self.connect_to_mage();
    }
}

#[godot_api]
impl ActionManager {
    fn action_loader(&mut self) -> Gd<SkillLoader> {
        self.base().get_node_as("SkillLoader")
    }

    fn player_vars(&mut self) -> Gd<PlayerVariables> {
        let tree = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        root.get_node_as("PlayerVars")
    }

    #[func]
    fn on_player_request_skill_action(&mut self, skill_index: i16, player_char_path: NodePath) {
        let player_vars = self.player_vars();
        let player_vars = player_vars.bind();
        let skill = player_vars.active_skills.get(&skill_index);

        if let Some(s) = skill {
            let instance = PlayerSkills::load_skill(s);

            if let Some(inst) = instance {
                let player_char = self.base().get_node(player_char_path);

                if let Some(mut char) = player_char {
                    char.add_child(inst);
                }
            }
        } else {
            godot_error!("skill index did not match skill");
        }
    }

    #[func]
    fn connect_to_mage(&mut self) {
        let tree = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let mut mage = root.get_node_as::<Mage>(NodePath::from("Main/Mage"));
        let mut mage = mage.bind_mut();
        let call = Callable::from_object_method(
            &self.base(),
            StringName::from("on_player_request_skill_action"),
        );
        mage.base_mut()
            .connect("player_requests_skill_action".into(), call);
    }
}

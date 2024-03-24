use godot::{engine::Window, prelude::*};

use crate::{
    enums::player_char_enums::skills::PlayerSkills, mage::Mage,
    resources::player_vars::PlayerVariableResource,
};

use super::{
    action_loader::{self, SkillLoader},
    character_variable_manager::PlayerVariableManager,
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionManager {
    #[export]
    player_var_path: NodePath,
    #[var]
    action_loader: Option<Gd<SkillLoader>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionManager {
    fn ready(&mut self) {
        self.player_var_path = NodePath::from("PlayerVarManager");
        self.connect_to_mage();
    }
}

#[godot_api]
impl ActionManager {
    fn tree(&self) -> Gd<SceneTree> {
        self.base().get_tree().unwrap()
    }

    fn root(&self) -> Gd<Window> {
        self.tree().get_root().unwrap()
    }

    fn action_loader(&mut self) -> Gd<SkillLoader> {
        self.base().get_node_as("SkillLoader")
    }

    fn player_vars(&mut self) -> Gd<PlayerVariableManager> {
        self.root().get_node_as(self.get_player_var_path())
    }

    #[func]
    fn on_player_request_skill_action(&mut self, skill_index: i16, player_char_path: NodePath) {
        let player_vars = self.player_vars();
        let player_var_manager = player_vars.bind();
        let player_var_resource = player_var_manager.get_mage_vars();

        if let Some(vars) = player_var_resource {
            let mage_vars = vars.bind();
            let skills = mage_vars.get_active_skills();
            let skill = skills.get(skill_index).expect("action manager line 48");
            let skill = skill.to::<PlayerSkills>();
            let required_spell_level = PlayerSkills::get_required_spellslots(&skill);
            let available_spellslots = mage_vars.get_remaining_spell_slots(required_spell_level);

            if available_spellslots != 0 {
                let skill_instance = PlayerSkills::load_skill(&skill);
                if let Some(instance) = skill_instance {
                    let player_char = self.base().get_node(player_char_path);

                    if let Some(mut char) = player_char {
                        char.add_child(instance);
                    }
                }
            }
        } else {
            godot_print!("does not have enough spell slots");
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

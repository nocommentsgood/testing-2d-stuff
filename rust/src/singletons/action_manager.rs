use godot::prelude::*;

use crate::{enums::player_char_enums::skills::PlayerSkills, mage::Mage};

use super::action_loader::{self, SkillLoader};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionManager {
    #[var]
    action_loader: Option<Gd<SkillLoader>>,
    #[export]
    mage: Option<Gd<Mage>>,
    #[export]
    mage_path: Option<Gd<Mage>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionManager {
    fn ready(&mut self) {
        let action_loader = self.base().get_node("SkillLoader".into()).unwrap();
        let action_loader = Some(action_loader.cast::<SkillLoader>());
        self.set_action_loader(action_loader);

        self.connect_node();
    }
}

#[godot_api]
impl ActionManager {
    fn action_loader(&mut self) -> Gd<SkillLoader> {
        self.base().get_node_as("SkillLoader")
    }

    #[func]
    fn on_player_skill_requested(
        &mut self,
        player_char_path: NodePath,
        skill_request: PlayerSkills,
    ) {
        godot_print!("func called by sig");
        let tree = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let mage = root.get_node_as::<Mage>(NodePath::from("Main/Mage"));

        let player_char = self.base().get_node(player_char_path);
        if let Some(mut char) = player_char {
            let skill = PlayerSkills::load_skill(&skill_request);

            if let Some(s) = skill {
                char.add_child(s);
            }
        }
    }

    #[func]
    fn connect_node(&mut self) {
        let tree = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let mut mage = root.get_node_as::<Mage>(NodePath::from("Main/Mage"));
        let mut mage = mage.bind_mut();
        let load = self.base().get_node_as::<SkillLoader>("SkillLoader");
        let call = Callable::from_object_method(
            &self.base(),
            StringName::from("on_player_skill_requested"),
        );
        mage.base_mut()
            .connect("player_spell_was_cast".into(), call, 0);
    }
}

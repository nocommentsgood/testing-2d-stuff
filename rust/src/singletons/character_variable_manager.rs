use godot::{engine::Window, prelude::*};

use crate::{
    enums::player_char_enums::playable_variables::PlayableCharVariables, mage::Mage,
    resources::player_vars::PlayerVariableResource,
};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct PlayerVariableManager {
    #[export]
    mage_vars: Option<Gd<PlayerVariableResource>>,
    #[export]
    fighter_vars: Option<Gd<PlayerVariableResource>>,
    #[export]
    path: NodePath,
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerVariableManager {
    fn ready(&mut self) {
        self.connect_to_player_characters();
    }
}

#[godot_api]
impl PlayerVariableManager {
    fn root(&mut self) -> Gd<Window> {
        let tree = self.base().get_tree().unwrap();
        tree.get_root().unwrap()
    }

    #[func]
    fn on_character_requests_level_1_skills(&mut self, path: NodePath) {}

    #[func]
    fn connect_to_player_characters(&mut self) {
        let root = self.root();
        let mut mage = root.get_node_as::<Mage>(NodePath::from("Main/Mage"));
        let mut mage = mage.bind_mut();
        let call = self
            .base()
            .callable(StringName::from("on_character_requests_level_1_skills"));
        mage.base_mut()
            .connect("request_level_1_skills".into(), call);

        let call2 = self
            .base()
            .callable(StringName::from("do_something_with_resource"));
        mage.base_mut().connect("request_health".into(), call2);
    }

    #[func]
    fn do_something_with_resource(&mut self) {
        let v = self.get_mage_vars().unwrap();
        let vo = v.try_cast::<PlayerVariableResource>();
        if let Ok(x) = vo {
            let vars = x.bind();
            godot_print!("health is : {}", vars.get_health());
        }
    }

    #[func]
    fn huh(&mut self) {
        let h = self.on_player_var_requested(PlayableCharVariables::HEALTH);
    }

    fn on_player_var_requested(&mut self, resource_type: PlayableCharVariables) -> Option<Variant> {
        let vars = self.get_mage_vars().unwrap();
        let vars = vars.try_cast::<PlayerVariableResource>();
        if let Ok(vars) = vars {
            let vars = vars.bind();

            let x = match resource_type {
                PlayableCharVariables::HEALTH => Some(vars.get_health().to_variant()),
                PlayableCharVariables::MAX_MOVEMENT_PER_TURN => {
                    Some(vars.get_max_movement_per_turn().to_variant())
                }
                PlayableCharVariables::ACTIVE_SKILLS => Some(vars.get_active_skills().to_variant()),
                PlayableCharVariables::ACTIONS => Some(vars.get_actions().to_variant()),
                PlayableCharVariables::BONUS_ACTIONS => Some(vars.get_bonus_actions().to_variant()),
                PlayableCharVariables::LEVEL_1_SPELL_SLOTS => {
                    Some(vars.get_level_1_spell_slots().to_variant())
                }
                PlayableCharVariables::LEVEL_2_SPELL_SLOTS => {
                    Some(vars.get_level_2_spell_slots().to_variant())
                }
                PlayableCharVariables::LEVEL_3_SPELL_SLOTS => {
                    Some(vars.get_level_3_spell_slots().to_variant())
                }
                PlayableCharVariables::LEVEL_4_SPELL_SLOTS => {
                    Some(vars.get_level_4_spell_slots().to_variant())
                }
                PlayableCharVariables::LEVEL_5_SPELL_SLOTS => {
                    Some(vars.get_level_5_spell_slots().to_variant())
                }
                PlayableCharVariables::LEVEL_6_SPELL_SLOTS => {
                    Some(vars.get_level_6_spell_slots().to_variant())
                }
                PlayableCharVariables::CHARISMA => Some(vars.get_charisma().to_variant()),
                PlayableCharVariables::WISDOM => Some(vars.get_wisdom().to_variant()),
                PlayableCharVariables::STRENGTH => Some(vars.get_strength().to_variant()),
                PlayableCharVariables::DEXTERITY => Some(vars.get_dexterity().to_variant()),
                PlayableCharVariables::CONSTITUTION => Some(vars.get_constitution().to_variant()),
            };
            x
        } else {
            None
        }
    }
}

use godot::{
    engine::{CharacterBody2D, InputEvent, Window},
    prelude::*,
};

use super::{action_manager::ActionManager, character_variable_manager::PlayerVariableManager};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameStateManager {
    #[export]
    combat_chain: Array<Gd<CharacterBody2D>>,
    is_state_default: bool,
    base: Base<Node>,
}

#[godot_api]
impl INode for GameStateManager {
    //     fn input(&mut self, event: Gd<InputEvent>) {
    //         if event.is_action_pressed("enter_turn_based".into()) {
    //             if self.is_state_default {
    //                 self.set_gamestate_to_turn_based();
    //                 self.is_state_default = false;
    //             } else {
    //                 self.set_gamestate_to_default();
    //                 self.is_state_default = true;
    //             }
    //         }
    //     }

    fn ready(&mut self) {
        self.is_state_default = true;
    }
}

#[godot_api]
impl GameStateManager {
    #[signal]
    fn changed_gamestate_to_turn_based();

    #[signal]
    fn changed_gamestate_to_default();

    fn tree(&self) -> Gd<SceneTree> {
        self.base().get_tree().unwrap()
    }
    fn root(&self) -> Gd<Window> {
        self.tree().get_root().unwrap()
    }

    pub fn set_gamestate_to_turn_based(&mut self) {
        let mut tree = self.tree();
        let player_group = tree.get_nodes_in_group("playercharacters".into());
        tree.call_group(
            "playercharacters".into(),
            "set_state_to_turn_based".into(),
            &[],
        );

        tree.call_group("enemy".into(), "set_state_to_turn_based".into(), &[]);
        self.base_mut()
            .emit_signal("changed_gamestate_to_turn_based".into(), &[]);
    }

    fn on_player_entered_combat(&mut self) {
        self.set_gamestate_to_turn_based();
    }

    fn set_gamestate_to_default(&mut self) {
        let mut tree = self.tree();
        let player_group = tree.get_nodes_in_group("playercharacters".into());
        tree.call_group(
            "playercharacters".into(),
            "set_state_to_default".into(),
            &[],
        );

        tree.call_group("enemy".into(), "set_state_to_default".into(), &[]);

        self.base_mut()
            .emit_signal("set_gamestate_to_default".into(), &[]);
    }
}

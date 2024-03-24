use godot::{
    engine::{InputEvent, Window},
    prelude::*,
};

use super::{action_manager::ActionManager, character_variable_manager::PlayerVariableManager};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameStateManager {
    #[export]
    action_manager_path: NodePath,
    #[export]
    action_manager: Option<Gd<ActionManager>>,
    #[export]
    var_manager_path: NodePath,
    #[export]
    var_manager: Option<Gd<PlayerVariableManager>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for GameStateManager {
    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("enter_turn_based".into()) {
            self.set_gamestate_to_turn_based();
        }
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl GameStateManager {
    #[signal]
    fn changed_gamestate_to_turn_based();

    fn tree(&self) -> Gd<SceneTree> {
        self.base().get_tree().unwrap()
    }
    fn root(&self) -> Gd<Window> {
        self.tree().get_root().unwrap()
    }

    pub fn set_gamestate_to_turn_based(&mut self) {
        let mut tree = self.tree();
        godot_print!("trying to set state");
        let player_group = tree.get_nodes_in_group("playercharacters".into());
        godot_print!("got group: {}", player_group);
        tree.call_group(
            "playercharacters".into(),
            "set_state_to_turn_based".into(),
            &[],
        );

        tree.call_group("enemy".into(), "set_state_to_turn_based".into(), &[]);
    }

    fn on_player_entered_combat(&mut self) {
        self.set_gamestate_to_turn_based();
        self.base_mut()
            .emit_signal("changed_gamestate_to_turn_based".into(), &[]);
    }
}

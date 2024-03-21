use godot::{
    engine::{EditorPlugin, IEditorPlugin},
    prelude::*,
};

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
pub struct GlobalState {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl GlobalState {
    const PLAYER_VARS: &'static str = "PlayerVars";
    const ACTION_LOADER: &'static str = "SkillHandler";
    const ACTION_MANAGER: &'static str = "ActionStateManager";
    const CHAR_VAR_MANAGER: &'static str = "PlayerVarManager";
}

#[godot_api]
impl IEditorPlugin for GlobalState {
    fn enter_tree(&mut self) {
        self.base_mut().add_autoload_singleton(
            GlobalState::PLAYER_VARS.into(),
            GString::from("res://scenes/player_variables.tscn"),
        );
        self.base_mut().add_autoload_singleton(
            GlobalState::ACTION_LOADER.into(),
            GString::from("res://singletons/skill_loader.tscn"),
        );
        self.base_mut().add_autoload_singleton(
            GlobalState::ACTION_MANAGER.into(),
            GString::from("res://singletons/action_manager.tscn"),
        );
        self.base_mut().add_autoload_singleton(
            GlobalState::CHAR_VAR_MANAGER.into(),
            GString::from("res://singletons/player_variable_manager.tscn"),
        );
    }

    fn exit_tree(&mut self) {
        self.base_mut()
            .remove_autoload_singleton(GlobalState::PLAYER_VARS.into());
        self.base_mut()
            .remove_autoload_singleton(GlobalState::ACTION_LOADER.into());
        self.base_mut()
            .remove_autoload_singleton(GlobalState::ACTION_MANAGER.into());
        self.base_mut()
            .remove_autoload_singleton(GlobalState::CHAR_VAR_MANAGER.into());
    }
}

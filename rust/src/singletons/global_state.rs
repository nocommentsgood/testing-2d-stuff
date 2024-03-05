use godot::{
    engine::{CharacterBody2D, NodeExt},
    prelude::*,
};

use crate::{mage::Mage, player_char_enums::CharacterState, spells::fireball::FireballSpell};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct PlayerVariables {
    #[export]
    #[init(default = 1.0)]
    this: real,
    fire_scene: Gd<PackedScene>,
    base: Base<Node>,
}

#[godot_api]
impl PlayerVariables {
    pub fn cast_player_spell(&mut self) {
        let root = self.base().get_tree().unwrap().get_root().unwrap();
        let mut fire = self.fire_scene.instantiate_as::<FireballSpell>();
        let mage = root.get_node_as::<Mage>("Main/Mage");
        let mut main = root.get_node_as::<Node2D>("Main");
        fire.set_global_position(mage.get_position());
        main.add_child(fire.clone().upcast());
    }
}

#[godot_api]
impl INode for PlayerVariables {
    fn ready(&mut self) {
        self.fire_scene = load("res://scenes/animations/spells/fire_ball.tscn");
    }
}

use godot::{
    engine::{CharacterBody2D, NodeExt},
    prelude::*,
};

use crate::{mage::Mage, spells::fireball::FireballSpell};

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
    pub fn test_tree(&mut self) {
        let tree = self.base().get_tree().unwrap();
        let spells = tree.get_root().unwrap();
        godot_print!("from autoload");
    }

    pub fn make_mage_cast(&mut self) {
        let mut fire = self.fire_scene.instantiate_as::<FireballSpell>();
        let root = self.base().get_tree().unwrap().get_root().unwrap();
        godot_print!("{}", root.get_children());
        let mut mage = root.get_node_as::<Mage>("Main/Mage");
        mage.add_child(fire.clone().upcast());
    }
}

#[godot_api]
impl INode for PlayerVariables {
    fn ready(&mut self) {
        self.fire_scene = load("res://scenes/animations/spells/fire_ball.tscn");
    }
}

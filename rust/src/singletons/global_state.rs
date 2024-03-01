use godot::{engine::CharacterBody2D, prelude::*};

use crate::mage::Mage;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct PlayerVariables {
    #[export]
    #[init(default = 1.0)]
    this: real,
    base: Base<Node>,
}

#[godot_api]
impl PlayerVariables {
    pub fn test_tree(&mut self) {
        let tree = self.base().get_tree().unwrap();
        let spells = tree.get_root().unwrap();
        godot_print!("from autoload");
    }

    // pub fn make_mage_cast(&mut self) {
    //     let tree = self.base().get_tree().unwrap();
    //     let root = tree.get_root().unwrap();
    //     let mut mage = root.get_node_as::<Mage>("Mage");
    //     mage.add_child(self.player_spells.instantiate().unwrap());
    //     godot_print!("{mage}");
    // }
}

// #[godot_api]
// impl TestingAutoload {
//     #[func]
//     fn hello(&mut self) {
//         godot_print!("Hello from singleton");
//     }
// }

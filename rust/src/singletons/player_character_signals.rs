use godot::prelude::*;

use crate::mage::Mage;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct PlayerCharSignalManager {
    #[export]
    path: Option<Gd<Node>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerCharSignalManager {
    fn ready(&mut self) {
        self.path = self.base().get_node("Main/Mage".into());
    }
}

#[godot_api]
impl PlayerCharSignalManager {
    #[signal]
    fn player_request_something();

    #[func]
    fn do_something_with_path(&mut self) {
        let mage = self.get_path().unwrap().cast::<Mage>();
    }
}

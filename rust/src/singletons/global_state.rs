use godot::prelude::*;

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
        let count = tree.get_node_count();
        godot_print!("{count}");
    }
}

// #[godot_api]
// impl TestingAutoload {
//     #[func]
//     fn hello(&mut self) {
//         godot_print!("Hello from singleton");
//     }
// }

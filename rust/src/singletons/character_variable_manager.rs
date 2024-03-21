use godot::{engine::Window, prelude::*};

use crate::{mage::Mage, resources::player_vars::PlayerVariableResource};

#[derive(GodotClass)]
#[class(base = Node)]
struct PlayerVariableManager {
    #[export]
    vars: Option<Gd<Node>>,
    #[export]
    path: NodePath,
    base: Base<Node>,
}

#[godot_api]
impl INode for PlayerVariableManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            vars: None,
            path: NodePath::default(),
            base,
        }
    }
    fn ready(&mut self) {
        self.vars = Some(self.base().get_property());
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
        let v = self.get_vars().unwrap();
        let vo = v.();
        godot_print!("health is: {}", vo.get_health());
    }
}

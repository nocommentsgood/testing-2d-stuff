use godot::{
    engine::{Button, CanvasLayer, CharacterBody2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl Main {
    // #[func]
    // fn on_spell_button_clicked(&mut self) {
    //     let mut button = self
    //         .base_mut()
    //         .get_node_as::<Button>("HUD/SpellContainer/GridContainer/Button");
    //     let mage = self.base_mut().get_node_as::<CharacterBody2D>("Mage");
    //
    //     let auto = self
    //         .base()
    //         .get_node_as::<PlayerVariables>("/root/PlayerVars");
    //     godot_print!("{auto}");
    //     godot_print!("{}", auto.bind().get_this());
    //
    //     button.connect(
    //         "spell_button_clicked".into(),
    //         mage.callable("on_spell_button_clicked"),
    //     );
    // }

    // #[func]
    // fn say_hello(&self) {
    //     let auto = self
    //         .base()
    //         .get_node_as::<TestingAutoload>("TestingAutoload");
    //     auto.bind().hello();
    // }
}

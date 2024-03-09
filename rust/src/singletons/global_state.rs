use godot::{
    engine::{Area2D, CharacterBody2D, NodeExt},
    prelude::*,
};

use crate::{
    enums::player_char_enums::skills::PlayerSkills, mage::Mage, spells::fireball::FireballSpell,
};

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

    pub fn cast_skill(&mut self, skill: PlayerSkills) {
        let root = self.base().get_tree().unwrap().get_root().unwrap();
        let mage = root.get_node_as::<Mage>("Main/Mage");
        let mut main = root.get_node_as::<Node2D>("Main");
        let skill_scene = PlayerSkills::load_action(skill);
        let mut instanced_skill = skill_scene.unwrap().instantiate_as::<Area2D>();
        instanced_skill.set_global_position(mage.get_position());
        main.add_child(instanced_skill.clone().upcast());
    }
}

#[godot_api]
impl INode for PlayerVariables {
    fn ready(&mut self) {
        self.fire_scene = load("res://scenes/animations/spells/fire_ball.tscn");
    }
}

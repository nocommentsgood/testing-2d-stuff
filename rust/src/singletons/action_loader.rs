use godot::prelude::*;

use crate::{
    enums::player_char_enums::skills::PlayerSkills,
    mage::Mage,
    spells::{fireball::FireballSpell, test_spell::TestSpell},
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SkillLoader {
    #[export]
    path: NodePath,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillLoader {
    fn ready(&mut self) {}
}

#[godot_api]
impl SkillLoader {
    pub fn cast_player_skill(&mut self, skill: (Option<Gd<Node>>, PlayerSkills)) {
        match skill {
            (Some(node), PlayerSkills::FIREBALL) => {
                let root = self.base().get_tree().unwrap().get_root().unwrap();
                let fire_scene = node.try_cast::<FireballSpell>();
                let player_char = root.get_node_as::<Mage>("Main/Mage");
                let mut main = root.get_node_as::<Node2D>("Main");
                if let Ok(mut fireball) = fire_scene {
                    fireball.set_global_position(player_char.get_position());
                    main.add_child(fireball.clone().upcast());
                }
            }
            (Some(node), PlayerSkills::TEST_SPELL) => {
                let root = self.base().get_tree().unwrap().get_root().unwrap();
                let test_spell_scene = node.try_cast::<TestSpell>();
                let player_char = root.get_node_as::<Mage>("Main/Mage");
                let mut main = root.get_node_as::<Node2D>("Main");
                if let Ok(mut test_spell) = test_spell_scene {
                    test_spell.set_global_position(player_char.get_position());
                    main.add_child(test_spell.clone().upcast());
                }
            }

            _ => (),
        }
    }

    pub fn new_cast_player_skill(&mut self, path: NodePath, skill: PlayerSkills) {
        match skill {
            PlayerSkills::FIREBALL => {
                let root = self.base().get_tree().unwrap().get_root().unwrap();
                let player_char = self.base().get_node(path).unwrap().cast::<Node2D>();
                let mut main = root.get_node_as::<Node2D>("Main");
                let mut spell_scene =
                    load::<PackedScene>("res://scenes/animations/spells/fire_ball.tscn")
                        .instantiate_as::<FireballSpell>();
                spell_scene.set_global_position(player_char.get_position());
                main.add_child(spell_scene.clone().upcast());
            }

            PlayerSkills::TEST_SPELL => {
                let root = self.base().get_tree().unwrap().get_root().unwrap();
                let player_char = self.base().get_node(path).unwrap().cast::<Node2D>();
                let mut main = root.get_node_as::<Node2D>("Main");
                let mut spell_scene =
                    load::<PackedScene>("res://scenes/animations/spells/test_spell.tscn")
                        .instantiate_as::<TestSpell>();
                spell_scene.set_global_position(player_char.get_position());
                main.add_child(spell_scene.clone().upcast());
            }
            _ => {}
        }
    }
}

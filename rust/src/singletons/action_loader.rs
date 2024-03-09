use godot::prelude::*;

use crate::enums::player_char_enums::skills::PlayerSkills;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SkillLoader {
    available_skills: Array<Gd<PackedScene>>,
    selected_spells: Array<Gd<PackedScene>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SkillLoader {
    fn ready(&mut self) {}
}

#[godot_api]
impl SkillLoader {
    fn load_skill(skill: PlayerSkills) -> Option<Gd<PackedScene>> {
        match skill {
            PlayerSkills::FIREBALL => Some(load("res://animations/spells/fire_ball.tscn")),
            PlayerSkills::TEST_SPELL => Some(load("res://animations/spells/test_spell.tscn")),
            _ => None,
        }
    }
}

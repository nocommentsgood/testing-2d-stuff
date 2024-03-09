use godot::{engine::Area2D, prelude::*};

use crate::spells::fireball::FireballSpell;

#[allow(non_camel_case_types)]
pub enum PlayerSkills {
    FIREBALL,
    TEST_SPELL,
    MAIN_HAND_ATTACK,
}

impl PlayerSkills {
    pub fn load_skill<T>(skill: PlayerSkills) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Resource>,
    {
        match skill {
            PlayerSkills::FIREBALL => {
                let s: Gd<PackedScene> = load("res://animations/spells/fire_ball.tscn");
                let x = s.instantiate_as::<Area2D>();
                Some(x)
            }
            PlayerSkills::TEST_SPELL => {
                let pack = load("res://animations/spells/test_spell.tscn");
                let inst = pack.instantiate_as::<TestSpell>();
            }
            _ => None,
        }
    }

    pub fn load_action(skill: PlayerSkills) -> Option<Gd<PackedScene>> {
        match skill {
            PlayerSkills::FIREBALL => Some(load("res://animations/spells/fire_ball.tscn")),
            _ => None,
        }
    }
}

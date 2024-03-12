use godot::{
    engine::Area2D,
    obj::{
        bounds::{self, MemDynamic, MemManual},
        Bounds, UserClass,
    },
    prelude::*,
};

use crate::spells::my_spell::Spell;
use crate::spells::{fireball::FireballSpell, test_spell::TestSpell};

#[allow(non_camel_case_types)]
pub enum PlayerSkills {
    FIREBALL,
    TEST_SPELL,
    MAIN_HAND_ATTACK,
    NONE,
}

impl PlayerSkills {
    pub fn load_skill(skill: PlayerSkills) -> Option<Gd<dyn INode>> {
        match skill {
            PlayerSkills::FIREBALL => {
                let pack = load::<PackedScene>("res://animations/spells/fire_ball.tscn");
                let instant = pack.instantiate_as::<FireballSpell>();
                let p = Some(instant);
            }
            PlayerSkills::TEST_SPELL => {
                let pack = load::<PackedScene>("res://animations/spells/test_spell.tscn");
                let instant = pack.instantiate_as::<TestSpell>();
                Some(instant)
            }
            _ => None,
        }
    }

    pub fn load_action<T>(skill: PlayerSkills) -> super::bound_example::MyGd<T> {
        match skill {
            PlayerSkills::FIREBALL => {
                let pack = load::<PackedScene>("res://animations/spells/fire_ball.tscn");
                let instant = pack.instantiate();
                if let Some(i) = instant {
                    let x = i.try_cast::<FireballSpell>();
                    let v = x.unwrap();
                    v
                }
            }
            _ => None,
        }
    }

    pub fn load_skill_scene(skill: PlayerSkills) -> (Option<Gd<PackedScene>>, PlayerSkills) {
        match skill {
            PlayerSkills::FIREBALL => {
                let pack = load::<PackedScene>("res://animations/spells/fire_ball.tscn");
                (Some(pack), PlayerSkills::FIREBALL)
            }

            PlayerSkills::TEST_SPELL => {
                let pack = load::<PackedScene>("res://animations/spells/test_spell.tscn");
                (Some(pack), PlayerSkills::TEST_SPELL)
            }

            _ => (None, PlayerSkills::NONE),
        }
    }
}

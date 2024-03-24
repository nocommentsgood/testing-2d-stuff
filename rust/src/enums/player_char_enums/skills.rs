use godot::prelude::*;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(GodotConvert, Var, Export, Debug)]
#[godot(via = GString)]
pub enum PlayerSkills {
    FIREBALL,
    TEST_SPELL,
    MAIN_HAND_ATTACK,
    NONE,
}

impl PlayerSkills {
    pub fn load_skill(skill: &PlayerSkills) -> Option<Gd<Node>> {
        match skill {
            PlayerSkills::FIREBALL => {
                let s = load::<PackedScene>("res://scenes/animations/spells/fire_ball.tscn");
                let x = s.instantiate_as::<Node>();
                Some(x)
            }
            PlayerSkills::TEST_SPELL => {
                let pack = load::<PackedScene>("res://scenes/animations/spells/test_spell.tscn");
                let inst = pack.instantiate_as::<Node>();
                Some(inst)
            }
            _ => None,
        }
    }

    pub fn get_required_spellslots(skill: &PlayerSkills) -> i16 {
        match skill {
            PlayerSkills::FIREBALL => 3,
            PlayerSkills::TEST_SPELL => 2,
            _ => 0,
        }
    }
}

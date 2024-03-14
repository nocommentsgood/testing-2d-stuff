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

// impl PlayerSkills {
//     pub fn load_skill(skill: &PlayerSkills) -> (Option<Gd<Node>>, PlayerSkills) {
//         match skill {
//             PlayerSkills::FIREBALL => {
//                 let s = load::<PackedScene>("res://scenes/animations/spells/fire_ball.tscn");
//                 let x = s.instantiate_as::<Node>();
//                 (Some(x), PlayerSkills::FIREBALL)
//             }
//             PlayerSkills::TEST_SPELL => {
//                 let pack = load::<PackedScene>("res://scenes/animations/spells/test_spell.tscn");
//                 let inst = pack.instantiate_as::<Node>();
//                 (Some(inst), PlayerSkills::TEST_SPELL)
//             }
//             _ => (None, PlayerSkills::NONE),
//         }
//     }
// }

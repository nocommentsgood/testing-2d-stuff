use godot::prelude::*;

use crate::{enums::player_char_enums::skills::PlayerSkills, spells::fireball::FireballSpell};

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
    pub fn instantiate_skill(pack: Option<Gd<PackedScene>>, skill: PlayerSkills) {
        match skill {
            PlayerSkills::FIREBALL => {
                if let Some(scene) = pack {
                    let s = scene.instantiate_as::<FireballSpell>();
                }
            }
        }
    }
}

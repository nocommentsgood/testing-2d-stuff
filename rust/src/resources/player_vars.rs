use godot::prelude::*;

use crate::enums::player_char_enums::skills::PlayerSkills;

#[derive(GodotClass)]
#[class(base = Resource)]
pub struct PlayerVariableResource {
    #[export]
    health: i32,
    #[export]
    max_movement_per_turn: i32,
    #[export]
    active_skills: Dictionary,
    #[export]
    actions: i16,
    #[export]
    bonus_actions: i16,
    #[export]
    level_1_spell_slots: i16,
    #[export]
    level_2_spell_slots: i16,
    #[export]
    level_3_spell_slots: i16,
    #[export]
    level_4_spell_slots: i16,
    #[export]
    level_5_spell_slots: i16,
    #[export]
    level_6_spell_slots: i16,
    #[export]
    charisma: i16,
    #[export]
    wisdom: i16,
    #[export]
    strength: i16,
    #[export]
    dexterity: i16,
    #[export]
    constitution: i16,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for PlayerVariableResource {
    fn init(base: Base<Resource>) -> Self {
        Self {
            health: 0,
            max_movement_per_turn: 1000,
            active_skills: dict! {
                1: PlayerSkills::FIREBALL,
                2: PlayerSkills::TEST_SPELL,
            },
            actions: 1,
            bonus_actions: 1,
            level_1_spell_slots: 2,
            level_2_spell_slots: 1,
            level_3_spell_slots: 1,
            level_4_spell_slots: 1,
            level_5_spell_slots: 1,
            level_6_spell_slots: 1,
            charisma: 2,
            wisdom: 2,
            strength: 2,
            dexterity: 2,
            constitution: 2,
            base,
        }
    }
}

use godot::prelude::*;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(GodotConvert, Var, Export, Debug)]
#[godot(via = GString)]
pub enum PlayableCharVariables {
    HEALTH,
    MAX_MOVEMENT_PER_TURN,
    ACTIVE_SKILLS,
    ACTIONS,
    BONUS_ACTIONS,
    LEVEL_1_SPELL_SLOTS,
    LEVEL_2_SPELL_SLOTS,
    LEVEL_3_SPELL_SLOTS,
    LEVEL_4_SPELL_SLOTS,
    LEVEL_5_SPELL_SLOTS,
    LEVEL_6_SPELL_SLOTS,
    CHARISMA,
    WISDOM,
    STRENGTH,
    DEXTERITY,
    CONSTITUTION,
}

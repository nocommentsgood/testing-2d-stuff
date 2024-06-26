#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(PartialEq, Default)]
pub enum CharacterState {
    #[default]
    DEFAULT,
    MOVING,
    CASTING_SPELL,
    LOOTING,
    TURN_BASED,
}

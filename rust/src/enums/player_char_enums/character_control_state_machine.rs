#[derive(PartialEq, Default)]
pub enum CharacterState {
    #[default]
    DEFAULT,
    MOVING,
    CASTING_SPELL,
    LOOTING,
}

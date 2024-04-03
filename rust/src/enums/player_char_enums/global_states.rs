#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(PartialEq, Default)]
pub enum GlobalState {
    #[default]
    DEFAULT,
    TURN_BASED,
}

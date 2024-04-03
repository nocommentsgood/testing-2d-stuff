use godot::{obj::WithBaseField, prelude::*};

pub trait RustNode: WithBaseField {
    fn tree(&self) -> Gd<SceneTree> {
        unimplemented!()
    }
}

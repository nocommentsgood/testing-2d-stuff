use godot::prelude::*;

#[derive(GodotClass, Default)]
#[class(init)]
pub struct DeleteMe {
    num: u32,
}

impl DeleteMe {
    pub fn p(&self) {
        godot_print!("num is: {}", self.num);
    }
}

use godot::{
    engine::{ILine2D, InputEvent, InputEventMouseMotion, Line2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base = Line2D)]
struct SkillDistanceLine {
    active: bool,

    #[var]
    target: Vector2,

    base: Base<Line2D>,
}

#[godot_api]
impl ILine2D for SkillDistanceLine {
    fn input(&mut self, input: Gd<InputEvent>) {
        if let Ok(input) = input.try_cast::<InputEventMouseMotion>() {
            godot_print!("got mouse motion");
            self.get_input(input);
        }
    }
}

#[godot_api]
impl SkillDistanceLine {
    fn get_input(&mut self, input: Gd<InputEventMouseMotion>) {
        self.set_target(input.get_global_position());
        let t = self.get_target();
        self.base_mut().set_point_position(1, t);
    }
}

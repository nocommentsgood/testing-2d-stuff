use godot::prelude::*;

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

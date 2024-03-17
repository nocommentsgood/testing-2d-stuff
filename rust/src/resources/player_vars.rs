use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Resource)]
struct PlayerVariableResource {
    #[export]
    health: i32,
    #[export]
    max_movement_per_turn: i32,
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
    #[export]
    skill_points: i16,
    #[export]
    max_skill_points: i16,
    #[export]
    level: i16,
    #[export]
    max_level: i16,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for PlayerVariableResource {
    fn init(base: Base<Resource>) -> Self {
        Self {
            health: 0,
            max_movement_per_turn: 1000,
            actions: 1,
            bonus_actions: 1,
            level_1_spell_slots: 1,
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
            skill_points: 8,
            max_skill_points: 30,
            level: 1,
            max_level: 20,
            base,
        }
    }
}

#[godot_api]
impl PlayerVariableResource {
    fn level_up(&mut self) {
        if self.get_level() < 20 {
            self.set_level(self.get_level() + 1);
            if self.get_level() % 2 == 0 {
                self.set_skill_points(self.get_skill_points() + 2)
            } else {
                self.set_skill_points(self.get_skill_points() + 1)
            }
        }
    }
}

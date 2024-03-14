use std::collections::HashMap;

use crate::{
    enums::player_char_enums::{
        character_control_state_machine::CharacterState, skills::PlayerSkills,
    },
    singletons::{action_loader::SkillLoader, character_variables::PlayerVariables},
};
use godot::{
    engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Mage {
    #[export]
    speed: real,
    #[var]
    target: Vector2,
    state: CharacterState,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Mage {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 200.0,
            target: Vector2::ZERO,
            state: CharacterState::DEFAULT,
            base,
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("click".into()) {
            self.target = self.base().get_global_mouse_position();
        }
        let mut viewport = self.base().get_viewport().unwrap();
        viewport.set_input_as_handled();
    }

    fn physics_process(&mut self, _delta: f64) {
        if self.state == CharacterState::DEFAULT || self.state == CharacterState::MOVING {
            let velocity = self.base().get_position().direction_to(self.target) * self.speed;
            let mut animated_sprite = self
                .base_mut()
                .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

            self.base_mut().set_velocity(velocity);

            if self.base().get_position().distance_to(self.target) > 10.0 {
                self.base_mut().move_and_slide();
            }

            let animation;
            if velocity.x > 0.0 && velocity.y > 0.0 {
                animation = "right_down";
            } else if velocity.x < 0.0 && velocity.y > 0.0 {
                animation = "left_down";
            } else if velocity.x < 0.0 && velocity.y < 0.0 {
                animation = "left_up";
            } else {
                animation = "right_up";
            }

            animated_sprite.play_ex().name(animation.into()).done();
        }
    }
}

#[godot_api]
impl Mage {
    #[func]
    fn on_spellbutton_pressed(&mut self, toggled: bool, spell_index: i16) {
        self.cast_spell_action(toggled, spell_index);
    }

    #[func]
    pub fn cast_spell_action(&mut self, toggled: bool, spell_index: i16) {
        if toggled {
            self.state = CharacterState::CASTING_SPELL;
            let player_vars = self
                .base()
                .get_node_as::<PlayerVariables>("/root/PlayerVars");
            let char_state = player_vars.bind();
            let skill = char_state.active_skills.get(spell_index).unwrap();

            let path = self.base().get_path();

            self.base_mut().emit_signal(
                "player_spell_was_cast".into(),
                &[path.to_variant(), skill.to_variant()],
            );
        } else {
            self.state = CharacterState::DEFAULT
        }
    }

    #[signal]
    pub fn player_spell_was_cast(&self, path: NodePath, skill: PlayerSkills);

    #[func]
    fn spell_was_cancelled(&mut self) {
        self.state = CharacterState::DEFAULT
    }
}

use crate::{
    enums::player_char_enums::{
        character_control_state_machine::CharacterState, playable_variables::PlayableCharVariables,
    },
    traits::characters::playable_character::Playable,
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
    #[export]
    max_movement_per_turn: f32,
    #[export]
    movement_left: f32,
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
            max_movement_per_turn: 1000.0,
            movement_left: 100.0,
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
            self.move_to_target(self.target);
        }
        if self.state == CharacterState::TURN_BASED {
            self.turn_based_move_to_target(self.target);
        }
    }
}

#[godot_api]
impl Mage {
    #[func]
    fn on_spellbutton_pressed(&mut self, toggled: bool, skill_index: i16) {
        self.cast_spell_action(toggled, skill_index);
    }

    #[func]
    pub fn cast_spell_action(&mut self, toggled: bool, skill_index: i16) {
        if toggled {
            self.state = CharacterState::CASTING_SPELL;
            let path = self.base().get_path();

            self.base_mut().emit_signal(
                "player_requests_skill_action".into(),
                &[skill_index.to_variant(), path.to_variant()],
            );
        } else {
            self.state = CharacterState::DEFAULT
        }
    }

    #[signal]
    fn player_requests_skill_action(skill_index: i16, player_char_path: NodePath);

    #[func]
    fn spell_was_cancelled(&mut self) {
        self.state = CharacterState::DEFAULT
    }

    #[func]
    fn get_health(&mut self) {
        godot_print!("emitting health signal from mage");
        self.request_health();
    }

    #[signal]
    fn character_variable_request(resource_type: PlayableCharVariables);

    #[func]
    fn request_health(&mut self) {
        self.request_character_variable(PlayableCharVariables::HEALTH);
    }

    #[func]
    fn set_state_to_turn_based(&mut self) {
        self.state = CharacterState::TURN_BASED;
    }
}

impl Playable for Mage {
    fn request_character_variable(
        &mut self,
        variable: crate::enums::player_char_enums::playable_variables::PlayableCharVariables,
    ) {
        self.base_mut().emit_signal(
            "character_variable_request".into(),
            &[variable.to_variant()],
        );
    }

    fn move_to_target(&mut self, target: Vector2) {
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

    fn turn_based_move_to_target(&mut self, target: Vector2) {
        let velocity = self.base().get_position().direction_to(self.target) * self.speed;
        let dist = self.base().get_position().distance_to(self.target);
        let mut animated_sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.base_mut().set_velocity(velocity);

        if dist > 10.0 && dist < self.get_max_movement_per_turn() && self.get_movement_left() != 0.0
        {
            let movement = self.get_movement_left();

            self.base_mut().move_and_slide();
            self.set_movement_left(movement - dist);
        } else {
            godot_print!("cannot move that far");
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

    fn set_state_to_turn_based(&mut self) {
        godot_print!("setting mage state to turn");
        self.state = CharacterState::TURN_BASED;
    }
}

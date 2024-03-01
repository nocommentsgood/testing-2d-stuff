use godot::{
    engine::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, InputEvent},
    prelude::*,
};

use crate::{global_state::PlayerVariables, player_char_enums::CharacterState};

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

    fn ready(&mut self) {
        let mut effect = self.base_mut().get_node_as::<Node2D>("Spell");
        effect.set_visible(false);
    }
}

#[godot_api]
impl Mage {
    #[func]
    fn cast_spell_action(&mut self, toggled: bool, _spell_index: real) {
        if toggled {
            self.state = CharacterState::CASTING_SPELL
        } else {
            self.state = CharacterState::DEFAULT
        }
        let mut auto = self
            .base()
            .get_node_as::<PlayerVariables>("/root/PlayerVars");
        auto.bind_mut().test_tree();
        let mut effect = self.base_mut().get_node_as::<Node2D>("Spell");
        effect.set_visible(toggled);
    }
}

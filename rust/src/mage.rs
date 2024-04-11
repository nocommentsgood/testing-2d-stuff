use crate::enums::player_char_enums::{
    character_control_state_machine::CharacterState, playable_variables::PlayableCharVariables,
};
use crate::singletons::action_manager::ActionManager;
use crate::traits::characters::playable_character::PlayablePointer;
use crate::traits::delete_me::DeleteMe;
use crate::traits::utils::input_handling::input_command::Command;
use crate::traits::{
    characters::playable_character::Playable, damageable::Damageable, health::Health,
};
use crate::utilities::inputs::input_handler::{self, InputHandler};
use godot::obj::NewAlloc;
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
    command_queue: Vec<Box<dyn Command>>,

    // TODO: this is just for testing
    health: u16,
    manager: Option<Gd<DeleteMe>>,

    state: CharacterState,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Mage {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 200.0,
            max_movement_per_turn: 500.0,
            movement_left: 500.0,
            target: Vector2::ZERO,
            command_queue: Vec::new(),
            health: 100,
            manager: None,
            state: CharacterState::DEFAULT,
            base,
        }
    }

    fn ready(&mut self) {
        self.manager = Some(Gd::from_object(
            crate::traits::delete_me::DeleteMe::default(),
        ));
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let command = InputHandler::handle_input(event);
        if let Some(mut command) = command {
            command.execute(self);
        }
        let mut viewport = self.base().get_viewport().unwrap();
        viewport.set_input_as_handled();
    }

    fn physics_process(&mut self, _delta: f64) {
        godot_print!("calling from process");
        self.move_to_target();
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

    #[signal]
    fn character_variable_request(resource_type: PlayableCharVariables);

    #[func]
    fn set_state_to_turn_based(&mut self) {
        self.state = CharacterState::TURN_BASED;
    }

    #[func]
    fn set_state_to_default(&mut self) {
        self.state = CharacterState::DEFAULT;
    }
}

impl Playable for Mage {
    fn move_to_target(&mut self) {
        let target = self.get_mouse();
        if self.state == CharacterState::DEFAULT {
            godot_print!("state is default");
            godot_print!("target is {}", target);

            if self.base().get_global_position() != target {
                godot_print!("self position is: {}", self.base().get_global_position());
                self.state = CharacterState::MOVING;
                godot_print!("state is moving");
                let velocity = self.base().get_position().direction_to(target) * self.speed;
                let mut animated_sprite = self
                    .base_mut()
                    .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

                self.base_mut().set_velocity(velocity);

                if self.base().get_position().distance_to(target) > 10.0 {
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

    fn turn_based_move_to_target(&mut self, target: Vector2) {
        godot_print!("turn moving to target WOW");
        let velocity = self.base().get_position().direction_to(self.target) * self.speed;
        let dist = self.base().get_position().distance_to(self.target);
        let cur_pos = self.base().get_position();
        let movement = self.get_movement_left();
        let mut animated_sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.base_mut().set_velocity(velocity);

        if dist > 10.0 && self.get_movement_left() > 0.0 && dist <= self.get_max_movement_per_turn()
        {
            self.base_mut().move_and_slide();
            let position = self.base().get_position();
            self.set_movement_left(movement - ((cur_pos - position).length()));
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

    fn get_mouse(&mut self) -> Vector2 {
        self.base().get_global_mouse_position()
    }
}

impl PlayablePointer for Gd<Mage> {
    fn request_character_variable(
        &mut self,
        variable: crate::enums::player_char_enums::playable_variables::PlayableCharVariables,
    ) {
        self.bind_mut().base_mut().emit_signal(
            "character_variable_request".into(),
            &[variable.to_variant()],
        );
    }
}

impl Health for Gd<Mage> {
    fn health(&self) -> u16 {
        self.bind().health
    }

    fn restore_health(&mut self, amount: u16) {
        self.bind_mut().health += amount;
    }
}

impl Damageable for Gd<Mage> {
    fn take_damage(&mut self, amount: u16) {
        self.bind_mut().health -= amount;
    }

    fn t_damage(entity: &dyn Health) {}
}

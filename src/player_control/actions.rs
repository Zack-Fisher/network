//this isn't even necessarily the player actions.
//it's just all the actions used in the game, including ui stuff that's not necessarily related

use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

/// Configures [`Actions`], the resource that holds all player input.
/// Add new input in [`set_actions`] and in [`game_control::generate_bindings!`](game_control).
pub struct ActionsPlugin;

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct ActionsFrozen {
    freeze_count: usize,
}
impl ActionsFrozen {
    pub fn freeze(&mut self) {
        self.freeze_count += 1;
    }
    pub fn unfreeze(&mut self) {
        self.freeze_count -= 1;
    }
    pub fn is_frozen(&self) -> bool {
        self.freeze_count > 0
    }
}

use crate::ui::UIState;

use super::action_handler::*;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerAction>()
            .register_type::<UiAction>()
            .register_type::<ActionsFrozen>()
            .init_resource::<ActionsFrozen>()

            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<ToggleActions<PlayerAction>>()
            .add_system(freeze_player_input)

            .add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                remove_actions_when_frozen.after(InputManagerSystem::ManualControl),
            );
    }
}

/// if the ui is holding up focus, freeze all Player input on the input stream directly.
fn freeze_player_input (
    mut toggler: ResMut<ToggleActions<PlayerAction>>,

    ui_state: Res<State<UIState>>,
)
{
    match ui_state.current().clone() {
        UIState::NoFocus => {
            toggler.enabled = true;
        }
        _ => {
            toggler.enabled = false;
        }
    }
}

//i think these all need to be in the same struct. we probably have to serialize camera actions
//as well.
#[derive(Debug, Clone, Copy, Actionlike, Reflect, FromReflect, Default)]
pub enum PlayerAction {
    #[default]
    Move,
    Sprint,
    Jump,
    Interact,
    SpeedUpDialog,
    NumberedChoice1,
    NumberedChoice2,
    NumberedChoice3,
    NumberedChoice4,
    NumberedChoice5,
    NumberedChoice6,
    NumberedChoice7,
    NumberedChoice8,
    NumberedChoice9,
    NumberedChoice0,

    //camera actions
    Pan,
    Zoom,
}

impl PlayerAction {
    pub fn numbered_choice(index: u8) -> Self {
        match index {
            0 => PlayerAction::NumberedChoice0,
            1 => PlayerAction::NumberedChoice1,
            2 => PlayerAction::NumberedChoice2,
            3 => PlayerAction::NumberedChoice3,
            4 => PlayerAction::NumberedChoice4,
            5 => PlayerAction::NumberedChoice5,
            6 => PlayerAction::NumberedChoice6,
            7 => PlayerAction::NumberedChoice7,
            8 => PlayerAction::NumberedChoice8,
            9 => PlayerAction::NumberedChoice9,
            _ => panic!(
                "Numbered choice index out of range: got {}, expected 0-9",
                index
            ),
        }
    }
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub enum UiAction {
    #[default]
    TogglePause,
    ToggleMap,
    ToggleChat,
    FocusChat,
}

pub fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Space, PlayerAction::Jump),
            (QwertyScanCode::LShift, PlayerAction::Sprint),
            (QwertyScanCode::E, PlayerAction::Interact),
            (QwertyScanCode::Space, PlayerAction::SpeedUpDialog),
            (QwertyScanCode::Key1, PlayerAction::NumberedChoice1),
            (QwertyScanCode::Key2, PlayerAction::NumberedChoice2),
            (QwertyScanCode::Key3, PlayerAction::NumberedChoice3),
            (QwertyScanCode::Key4, PlayerAction::NumberedChoice4),
            (QwertyScanCode::Key5, PlayerAction::NumberedChoice5),
            (QwertyScanCode::Key6, PlayerAction::NumberedChoice6),
            (QwertyScanCode::Key7, PlayerAction::NumberedChoice7),
            (QwertyScanCode::Key8, PlayerAction::NumberedChoice8),
            (QwertyScanCode::Key9, PlayerAction::NumberedChoice9),
            (QwertyScanCode::Key0, PlayerAction::NumberedChoice0),
        ])
        //we can bind actions to axes.
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .insert(DualAxis::mouse_motion(), PlayerAction::Pan)
        .insert(SingleAxis::mouse_wheel_y(), PlayerAction::Zoom)
        .build(),
        ..default()
    }
}

pub fn create_ui_action_input_manager_bundle() -> InputManagerBundle<UiAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Escape, UiAction::TogglePause),
            (QwertyScanCode::U, UiAction::ToggleMap),
            (QwertyScanCode::Y, UiAction::ToggleChat),
            (QwertyScanCode::T, UiAction::FocusChat),
            ]),
        ..default()
    }
}

pub fn remove_actions_when_frozen(
    actions_frozen: Res<ActionsFrozen>,
    mut player_actions_query: Query<&mut ActionState<PlayerAction>>,
) {
    if actions_frozen.is_frozen() {
        for mut player_actions in player_actions_query.iter_mut() {
            player_actions.action_data_mut(PlayerAction::Move).axis_pair = Some(default());
            player_actions.release(PlayerAction::Jump);
            player_actions.release(PlayerAction::Interact);
            player_actions.release(PlayerAction::Sprint);
            player_actions.action_data_mut(PlayerAction::Pan).axis_pair = Some(default());
            player_actions.action_data_mut(PlayerAction::Zoom).value = default();
        }
    }
}

pub trait DualAxisDataExt {
    fn max_normalized(self) -> Option<Vec2>;
}

impl DualAxisDataExt for DualAxisData {
    fn max_normalized(self) -> Option<Vec2> {
        let vector = self.xy();
        let len_squared = vector.length_squared();
        if len_squared > 1.0 {
            Some(vector.normalize())
        } else if len_squared < 1e-5 {
            None
        } else {
            Some(vector)
        }
    }
}

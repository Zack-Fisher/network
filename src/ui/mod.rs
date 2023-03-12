use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, prelude::ActionState};

use crate::player_control::actions::{create_ui_action_input_manager_bundle, UiAction};

pub mod raceui;
pub mod mapui;
pub mod chat;
pub mod voices;

//there will probably be many cases in which input is consumed by some action in the ui's focus.
//we'll make a state just for this, and put all of that in one enum instead of splitting across
//the ui processes.
/// only one elm can be focused at a time, and
/// it's not meaningful for some things to be focused.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum UIState {
    ChatFocus,
    NoFocus,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(UIState::NoFocus)

            .add_startup_system(make_ui_input_manager)

            .add_system(process_state_from_keyinput)

            .add_plugin(raceui::RaceUIPlugin)
            .add_plugin(mapui::MapUIPlugin)
            .add_plugin(chat::ChatPlugin)
            .add_plugin(voices::VoicesPlugin)
            ; 
    }
}

//define some constants for ui zindex layers to make visibility easier to wrangle.
const RACELAYER: ZIndex = ZIndex::Global(1);
const MAPLAYER: ZIndex = ZIndex::Global(2);
const CHATLAYER: ZIndex = ZIndex::Global(5);
const VOICESLAYER: ZIndex = ZIndex::Global(9);

fn make_ui_input_manager (
    mut commands: Commands,
)
{
    //don't despawn this. because of leafwing stuff, we need input to be attached to an entity
    //in the scene itself.
    commands
        .spawn(
            create_ui_action_input_manager_bundle()
        )
        ;
}

fn process_state_from_keyinput (
    mut ui_state: Res<State<UIState>>,

    uiaction_q: Query<&ActionState<UiAction>>,
)
{
    for actionstate in uiaction_q.iter() {
        if actionstate.pressed(
            UiAction::FocusChat
        ) {
            match ui_state.current() {
                UIState::ChatFocus => {info!("chat already focused");}
                _ => {ui_state.set(UIState::ChatFocus); info!("set chat as the ui focus");}
            }
        }
    }
}

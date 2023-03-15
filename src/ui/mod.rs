use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, prelude::ActionState};

use crate::player_control::{actions::{create_ui_action_input_manager_bundle, UiAction}, camera::ForceCursorGrabMode};

pub mod raceui;
pub mod mapui;
pub mod chat;
pub mod voices;
pub mod equip;
pub mod analyse;

//there will probably be many cases in which input is consumed by some action in the ui's focus.
//we'll make a state just for this, and put all of that in one enum instead of splitting across
//the ui processes.
/// only one elm can be focused at a time, and
/// it's not meaningful for some things to be focused.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum UIState {
    AnalyseMode,
    EquipFocus,
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
            .add_system(focused_process)

            .add_plugin(raceui::RaceUIPlugin)
            .add_plugin(mapui::MapUIPlugin)
            .add_plugin(chat::ChatPlugin)
            .add_plugin(voices::VoicesPlugin)
            .add_plugin(analyse::AnalysePlugin)
            .add_plugin(equip::EquipPlugin)
            ; 
    }
}

//define some constants for ui zindex layers to make visibility easier to wrangle.
const RACELAYER: ZIndex = ZIndex::Global(1);
const MAPLAYER: ZIndex = ZIndex::Global(2);
const CHATLAYER: ZIndex = ZIndex::Global(5);
const VOICESLAYER: ZIndex = ZIndex::Global(9);
const ANALYSELAYER: ZIndex = ZIndex::Global(10);
const EQUIPLAYER: ZIndex = ZIndex::Global(11);

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
    mut ui_state: ResMut<State<UIState>>,

    uiaction_q: Query<&ActionState<UiAction>>,
)
{
    for actionstate in uiaction_q.iter() {
        let mut is_anything_pressed = false;

        if actionstate.pressed(
            UiAction::Analyse
        )
        {
            is_anything_pressed = true;
            match ui_state.current() {
                UIState::AnalyseMode => {},
                _ => {ui_state.set(UIState::AnalyseMode); info!("entering analysemode");}
            }
        }

        if actionstate.just_pressed(
            UiAction::FocusChat
        ) {
            is_anything_pressed = true;
            match ui_state.current() {
                UIState::ChatFocus => {ui_state.set(UIState::NoFocus); info!("chat already focused, unfocusing...");}
                _ => {ui_state.set(UIState::ChatFocus); info!("set chat as the ui focus");}
            }
        }

        if actionstate.just_pressed(
            UiAction::ToggleEquip
        ) {
            is_anything_pressed = true;
            match ui_state.current() {
                UIState::EquipFocus => {ui_state.set(UIState::NoFocus); info!("equips already focused, unfocusing...");}
                _ => {ui_state.set(UIState::EquipFocus); info!("set chat as the ui focus");}
            }
        }

        if !is_anything_pressed {
            match ui_state.current() {
                UIState::AnalyseMode => {ui_state.set(UIState::NoFocus); info!("leaving analysemode");},
                _ => {}
            }
        }
    }
}

use bevy::window::CursorGrabMode;

fn focused_process (
    mut focus_res: ResMut<ForceCursorGrabMode>,

    ui_state: Res<State<UIState>>,
)
{
    match ui_state.current().clone() {
        UIState::NoFocus => {
            focus_res.0 = Some(CursorGrabMode::Locked)
        },
        _ => {
            focus_res.0 = Some(CursorGrabMode::Confined)
        }
    }
}

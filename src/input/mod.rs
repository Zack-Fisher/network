use bevy::prelude::*;
use std::collections::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;


use bevy::window::CursorGrabMode;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            //modify this with the startup system
            .insert_resource(InputMapping {key_map: HashMap::new(), mouse_map: HashMap::new()})
            .add_system(cursor_grab_system)
            .add_startup_system_to_stage(StartupStage::PreStartup, init_input_mapping);
    }
}

//pass around and read as a resource on the app
#[derive(Resource)]
pub struct InputMapping {
    key_map: HashMap<Action, KeyCode>,
    mouse_map: HashMap<Action, MouseButton>,
}

//action->one input.
//try to map each action to one input, to avoid Nones and option garbage

#[derive(Hash, PartialEq, Eq)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveDown,
    MoveUp,
    PlayerJump,
    OpenMap,
}

//have to pass the mutable mapping from whatever system happens to call this. makes sense
//be careful calling functions in crates directly like this? kind of weird pattern
pub fn get_key(
    //pass a pointer to the map so that we can use it multiple times per system.
    mapping: &Res<InputMapping>,
    action: Action,
) -> KeyCode
{
    //if there's more than one input per action, we panic here i guess.
    //just call this function, simple.
    //why the deref? isn't that unsafe? w/e
    mapping.key_map.get(&action).cloned().unwrap()
}

fn init_input_mapping(
    //can mutably or immutably borrow any resource off the line
    mut mapping: ResMut<InputMapping>,
)
{
    //keycode mapping, mouse button mapping.
    //serialize later, just init defaults here for now.
    //each action corresponds to at most one input
    mapping.key_map
        .insert(Action::MoveUp, KeyCode::W);
    mapping.key_map
        .insert(Action::MoveLeft, KeyCode::A);
    mapping.key_map
        .insert(Action::MoveDown, KeyCode::S);
    mapping.key_map
        .insert(Action::MoveRight, KeyCode::D);
    mapping.key_map
        .insert(Action::PlayerJump, KeyCode::Space);
    mapping.key_map
        .insert(Action::OpenMap, KeyCode::T);
}

fn insert_into_map
(
    mapping: Res<InputMapping>,
)
{

}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        window.set_cursor_grab_mode(CursorGrabMode::Confined);

        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        // also hide the cursor
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}
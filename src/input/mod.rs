use bevy::prelude::*;
use std::collections::*;

use bevy::window::CursorGrabMode;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ChangeKeyMapEvent>()
            .add_event::<ChangeMouseMapEvent>()

            //modify this with the startup system
            .insert_resource(InputMapping {key_map: HashMap::new(), mouse_map: HashMap::new()})
            // .add_system(cursor_grab_system)
            .add_startup_system_to_stage(StartupStage::PreStartup, init_input_mapping)
            .add_system(input_testers)
            .add_system(insert_into_keymap)
            .add_system(insert_into_mousemap)
            ;
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

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveDown,
    MoveUp,
    PlayerJump,
    OpenMap,
    Interact,

    //tester actions, not to be used in the actual game.
    TestLoad,
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


    //test mappings. remove in final build.
    mapping.key_map
        .insert(Action::TestLoad, KeyCode::P);
}

//map actions to keycodes. the system will take care of the rest, ideally.
pub struct ChangeKeyMapEvent {
    action: Action,
    key: KeyCode,
}

//we NEED to make sure that the inserted action appears exactly one time in the keymapping, or we're in trouble.
//do lots of silliness and tomfoolery here.
fn insert_into_keymap
(
    mut mapping: ResMut<InputMapping>,

    mut changei_evr: EventReader<ChangeKeyMapEvent>,
)
{
    for ev in changei_evr.iter() {
        //jank, also only applies for keys now.
        mapping.key_map.remove(&ev.action);
        mapping.key_map.insert(ev.action, ev.key);
    }
}

pub struct ChangeMouseMapEvent {
    action: Action,
    mouse_button: MouseButton,
}

//we NEED to make sure that the inserted action appears exactly one time in the keymapping, or we're in trouble.
//do lots of silliness and tomfoolery here.
fn insert_into_mousemap
(
    mut mapping: ResMut<InputMapping>,

    mut changei_evr: EventReader<ChangeMouseMapEvent>,
)
{
    for ev in changei_evr.iter() {
        //jank, also only applies for keys now.
        mapping.mouse_map.remove(&ev.action);
        mapping.mouse_map.insert(ev.action, ev.mouse_button);
    }
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

use crate::load::*;

//this is probably going to get bad. just please keep it encapsulated in the input module.
fn input_testers (
    keyboard: Res<Input<KeyCode>>,
    mapping: Res<InputMapping>,

    mut load_evw: EventWriter<LoadLevelEvent>,
) {
    if keyboard.just_pressed(get_key(&mapping, Action::TestLoad)) {
        info!("running a test scene load");
        load_evw.send(LoadLevelEvent::default());
    }
}
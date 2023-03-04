use bevy::prelude::*;

//add extra types to the selection menu, do it all here in this plugin.
pub struct DevTypesPlugin;

impl Plugin for DevTypesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_types)
            ; 
    }
}

use bevy_editor_pls::default_windows::add::*;
use bevy_editor_pls::*;

//grab the editor window state mutably and modify it to allow for extra addable types.
fn add_types (
    //need to grab the Editor type AFTER the EditorPlugin is done initting
    mut window: ResMut<Editor>,
) 
{
    let state = window.window_state_mut::<AddWindow>().unwrap();

    //the first param is the section of the dropdown the type will be added to.
    // state.add(
    //     "",
    //     AddItem::component::<T>(),
    // );
}

// a component needs the following proc macros applied to it to serialize properly into a .scn.ron file.
// #[derive(Default, Component, Reflect, Serialize, Deserialize)]
// #[reflect(Component, Serialize, Deserialize)]
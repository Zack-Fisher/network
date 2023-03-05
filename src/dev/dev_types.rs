use bevy::prelude::*;

//add extra types to the selection menu, do it all here in this plugin.
pub struct DevTypesPlugin;

impl Plugin for DevTypesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_types)
            //cannot change struct values from default if you can't register the type.
            //all structs should implement default

            ///// PREFAB OBJECTS /////
            .register_type::<PlayerPrefab>()
            .register_type::<CameraPrefab>()
            .register_type::<NPCPrefab>()
            .register_type::<SpawnpointPrefab>()
            .register_type::<WarppointPrefab>()
            /////

            ///// PRIMITIVES /////
            .register_type::<CubePrefab>()
            /////
            ; 
    }
}

//please add more as you see fit.
pub enum EditorCategories {
    Prefab,
    ThreeDUtil,
    TwoDUtil,
    UIUtil,
    //for temp components that are only concerned with editor-type stuff.
    EditorUtil,
    Rapier,
    Primitive,
}

impl EditorCategories {
    fn to_string(&self) -> &str {
        match self {
            EditorCategories::Prefab => "Prefab",
            EditorCategories::ThreeDUtil => "ThreeDUtil",
            EditorCategories::TwoDUtil => "TwoDUtil",
            EditorCategories::UIUtil => "UIUtil",
            EditorCategories::EditorUtil => "EditorUtil",
            EditorCategories::Rapier => "Rapier",
            EditorCategories::Primitive => "Primitive",
        }
    }
}

use bevy_editor_pls::default_windows::add::*;
use bevy_editor_pls::*;

use crate::level_instantiation::spawning::objects::camera::CameraPrefab;
use crate::level_instantiation::spawning::objects::player::PlayerPrefab;
use crate::level_instantiation::spawning::objects::npc::NPCPrefab;
use crate::level_instantiation::spawning::objects::primitives::CubePrefab;
use crate::level_instantiation::spawning::objects::spawnpoint::SpawnpointPrefab;
use crate::level_instantiation::spawning::objects::warppoint::WarppointPrefab;
use crate::player_control::player_embodiment::Player;

use crate::movement::general_movement::*;

//grab the editor window state mutably and modify it to allow for extra addable types.
fn add_types (
    //need to grab the Editor type AFTER the EditorPlugin is done initting
    mut window: ResMut<Editor>,
) 
{
    let state = window.window_state_mut::<AddWindow>().unwrap();

    //the first param is the section of the dropdown the type will be added to.
    // state.add(
    //     EditorCategories::Prefab.to_string(),
    //     AddItem::component::<T>(),
    // );

    //why wasn't this added in the first place???
    state.add(
        "",
        AddItem::bundle::<SpatialBundle>(),
    );

    state.add(
        "",
        AddItem::component::<CharacterAnimations>(),
    );

    //adding the prefab types
    state.add(
        EditorCategories::Prefab.to_string(),
        AddItem::component::<PlayerPrefab>(),
    );

    state.add(
        EditorCategories::Prefab.to_string(),
        AddItem::component::<CameraPrefab>(),
    );

    state.add(
        EditorCategories::Prefab.to_string(),
        AddItem::component::<NPCPrefab>(),
    );

    state.add(
        EditorCategories::Prefab.to_string(),
        AddItem::component::<WarppointPrefab>(),
    );

    state.add(
        EditorCategories::Prefab.to_string(),
        AddItem::component::<SpawnpointPrefab>(),
    );

    state.add(
        EditorCategories::Primitive.to_string(),
        AddItem::component::<CubePrefab>(),
    );
}

// a component needs the following proc macros applied to it to serialize properly into a .scn.ron file.
// #[derive(Default, Component, Reflect, Serialize, Deserialize)]
// #[reflect(Component, Serialize, Deserialize)]
//it also needs to be .register_type'd into the App object itself through the plugin.
//maybe consolidate this stuff into one .register() method? 
//i don't know exactly how to handle the app. i don't think register_type() can be called through commands, maybe use the World
//, borrow it as &mut World and run an exclusive system that registers types? can we pass generics over the event line? i doubt it.
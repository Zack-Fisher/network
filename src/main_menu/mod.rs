use bevy::prelude::*;

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(spawn_main_menu)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(despawn_main_menu)
            )
            ;
    }
}

#[derive(Component)]
struct MenuUIRoot;

fn spawn_main_menu(
    mut commands: Commands,
)
{

}

fn despawn_main_menu(
    mut commands: Commands,
    menu_q: Query<Entity, With<MenuUIRoot>>,
)
{
    for ent in menu_q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

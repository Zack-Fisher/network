use crate::file_system_interaction::game_state_serialization::{GameLoadRequest, GameSaveRequest};
use crate::file_system_interaction::level_serialization::WorldLoadRequest;
use crate::level_instantiation::level::Levels;
use crate::player_control::camera::ForceCursorGrabMode;
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_editor_pls::editor_window::EditorWindow;
use bevy_editor_pls::{AddEditorWindow, Editor, EditorEvent};
use bevy_egui::egui;
use bevy_egui::egui::ScrollArea;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::NavMesh;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub struct DevEditorPlugin;

impl Plugin for DevEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DevEditorState>()
            .add_editor_window::<DevEditorWindow>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(handle_debug_render.pipe(log_errors))
                    .with_system(handle_navmesh_render.pipe(log_errors))
                    .with_system(set_cursor_grab_mode),
            );
    }
}

pub struct DevEditorWindow;

impl EditorWindow for DevEditorWindow {
    type State = DevEditorState;
    const NAME: &'static str = "Foxtrot Dev";
    const DEFAULT_SIZE: (f32, f32) = (200., 150.);
    fn ui(
        world: &mut World,
        mut cx: bevy_editor_pls::editor_window::EditorWindowContext,
        ui: &mut egui::Ui,
    ) {
        let state = cx
            .state_mut::<DevEditorWindow>()
            .expect("Failed to get dev window state");

        state.open = true;
        ui.heading("Debug Rendering");
        ui.checkbox(&mut state.collider_render_enabled, "Colliders");
        ui.checkbox(&mut state.navmesh_render_enabled, "Navmeshes");
        ui.separator();

        ui.heading("Scene Control");
        ui.horizontal(|ui| {
            ui.label("Level name: ");
            ui.text_edit_singleline(&mut state.level_name);
        });

        egui::ComboBox::from_label("Pick a Level")
            .show_ui(ui, |ui| {
                for value in Levels::iter() {
                    ui.selectable_value(&mut state.selected, value.clone(), value.clone().to_string());    
                }
            }
            );

        if ui.button("Load level").clicked() {
            world.send_event( {
                WorldLoadRequest {
                    level: state.selected.clone(),
                    spawnpoint_name: String::from("not implemented"),
                }
            });
        }

        ui.add_space(3.);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct DevEditorState {
    pub open: bool,
    pub level_name: String,
    pub save_name: String,
    pub collider_render_enabled: bool,
    pub navmesh_render_enabled: bool,
    pub selected: Levels,
}

impl Default for DevEditorState {
    fn default() -> Self {
        Self {
            level_name: "old_town".to_owned(),
            save_name: default(),
            collider_render_enabled: false,
            navmesh_render_enabled: false,
            open: false,
            selected: Levels::default(),
        }
    }
}

fn handle_debug_render(
    state: Res<Editor>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) -> Result<()> {
    debug_render_context.enabled = state
        .window_state::<DevEditorWindow>()
        .context("Failed to read dev window state")?
        .collider_render_enabled;
    Ok(())
}

fn set_cursor_grab_mode(
    mut events: EventReader<EditorEvent>,
    mut force_cursor_grab: ResMut<ForceCursorGrabMode>,
) {
    for event in events.iter() {
        if let EditorEvent::Toggle { now_active } = event {
            if *now_active {
                force_cursor_grab.0 = Some(CursorGrabMode::None);
            } else {
                force_cursor_grab.0 = None;
            }
        }
    }
}

fn handle_navmesh_render(
    state: Res<Editor>,
    nav_mesh: Res<NavMesh>,
    mut lines: ResMut<DebugLines>,
) -> Result<()> {
    if !state
        .window_state::<DevEditorWindow>()
        .context("Failed to read dev window state")?
        .navmesh_render_enabled
    {
        return Ok(());
    }

    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (tile_coord, tile) in nav_mesh.get_tiles().iter() {
            let tile_color = Color::Rgba {
                red: 0.0,
                green: (tile_coord.x % 10) as f32 / 10.0,
                blue: (tile_coord.y % 10) as f32 / 10.0,
                alpha: 1.0,
            };
            // Draw polygons.
            for poly in tile.polygons.iter() {
                let indices = &poly.indices;
                for i in 0..indices.len() {
                    let a = tile.vertices[indices[i] as usize];
                    let b = tile.vertices[indices[(i + 1) % indices.len()] as usize];

                    lines.line_colored(a, b, 0.0, tile_color);
                }
            }

            // Draw vertex points.
            for vertex in tile.vertices.iter() {
                lines.line_colored(*vertex, *vertex + Vec3::Y, 0.0, tile_color);
            }
        }
    }
    Ok(())
}

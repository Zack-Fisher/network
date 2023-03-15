use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_picking::*;

pub mod button;

use crate::ui::UIState;

pub struct AnalysisPlugin;

impl Plugin for AnalysisPlugin {
    fn build(&self, app: &mut App) {
        app
            //maybe don't strictly relate modpicking to analyse mode?
            .add_plugins(DefaultPickingPlugins)

            .add_plugin(button::ButtonEvPlugin)

            //read by the ui system, this is the analysis system's endpoint.
            .insert_resource(CurrAnalysis::default())

            .add_system(analyse_process)
            .add_system(change_camera_layers)
            ;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum AnalysisMode {
    #[default]
    Info,
    Event,
}

#[derive(Resource, Default)]
pub struct CurrAnalysis {
    pub curr: Option<AnalysisData>,
}

#[derive(Default, Component, Clone)]
pub struct AnalysisData {
    pub title: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum AnalysisEventType {
    #[default]
    Button,
}

/// contains an arbitrary payload, usually loaded in by the nametag.
/// eg: [button=5] loads in the button call event with the payload string "5".
#[derive(Default, Component, Clone)]
pub struct AnalysisEvent {
    pub payload: String,
    pub ev_type: AnalysisEventType,
}

#[derive(Component, Default)]
pub struct Analysis {
    pub data: Option<AnalysisData>,
    pub event: Option<AnalysisEvent>,
}

#[derive(Bundle)]
pub struct AnalyseBundle {
    pub mod_picking: PickableBundle,
    pub analysis: Analysis,
    /// bevy mod picking uses a mesh to pick.
    pub pbr: PbrBundle,
    pub layers: RenderLayers,
}

impl Default for AnalyseBundle {
    fn default() -> Self {
        Self {
            mod_picking: PickableBundle::default(),
            analysis: Analysis::default(),
            pbr: PbrBundle::default(),
            //spawn them on a special layer by default
            layers: RenderLayers::from_layers(&[3]),
        }
    }
}

//change the camera's rendering layer to view the pbrbundle.
//this should usually only change in analyse mode.
impl From<PbrBundle> for AnalyseBundle {
    fn from(value: PbrBundle) -> Self {
        Self {
            pbr: value,
            ..default()
        }
    }
}

/// make the selectable meshes visible to the camera.
fn change_camera_layers (
    mut cam_q: Query<&mut RenderLayers, With<Camera>>,

    ui_state: Res<State<UIState>>,
)
{
    match ui_state.current().clone() {
        UIState::AnalyseMode => {
            for mut layers in cam_q.iter_mut() {
                *layers = layers.with(3);
            }
        },
        _ => {
            for mut layers in cam_q.iter_mut() {
                *layers = layers.without(3);
            }
        }
    }
}

fn analyse_process (
    mut picking_evr: EventReader<PickingEvent>,

    mut curr_an: ResMut<CurrAnalysis>,

    an_q: Query<&AnalysisData>,
)
{
    for ev in picking_evr.iter() {
        match ev.clone() {
            PickingEvent::Clicked(entity) => {
                info!("clicked {:?}", entity.clone());
                if let Ok(data) = an_q.get(entity.clone()) {
                    curr_an.curr = Some(data.clone());
                }
            },
            _ => {

            }
        }
    }
}

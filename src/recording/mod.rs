use bevy::prelude::*;

pub struct RecordingPlugin;

impl Plugin for RecordingPlugin {
    fn build(&self, app: &mut App) {
        app
            ;
    }
}

struct PlayerRecording {
    actions: Vec<PlayerAction>,
}

fn record_player_actions (
    mut player_action_q: Query<(&ActionState<PlayerAction>)>,
    mut camera_action_q: Query<(&ActionState<CameraAction>)>,

    time: Res<Time>,
)
{
    for player_actions in player_action_q.iter_mut() {
        
    }

    for camera_actions in camera_action_q.iter_mut() {

    }
}
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayBGMEvent>()
            .add_event::<PlaySFXEvent>()

            .add_audio_channel::<BGM>()
            .add_audio_channel::<SFX>()

            .add_system(play_bgm)
            ;
    }
}

#[derive(Resource)]
struct BGM;

#[derive(Resource)]
struct SFX;

pub struct PlayBGMEvent {
    path: String,
}

fn play_bgm(
    mut play_bgm_evr: EventReader<PlayBGMEvent>,
    server: Res<AssetServer>,

    audio: Res<AudioChannel<PlayBGMEvent>>,
) {
    for ev in play_bgm_evr.iter() {
        audio
            .play(server.load(ev.path.clone()))
            .fade_in(AudioTween::new(bevy::utils::Duration::from_secs(2), AudioEasing::OutPowi(2)))
            .with_playback_rate(1.5);
    }
}

pub struct PlaySFXEvent {
    path: String,
}

fn play_sfx(
    mut play_sfx_evr: EventReader<PlaySFXEvent>,
    server: Res<AssetServer>,

    audio: Res<AudioChannel<PlaySFXEvent>>,
) {
}

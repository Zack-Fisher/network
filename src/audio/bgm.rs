use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)

            .add_event::<PlayBGMEvent>()
            .add_event::<PlaySFXEvent>()

            .insert_resource(BGM)
            .insert_resource(SFX)

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

#[derive(Debug)]
pub struct PlayBGMEvent {
    pub path: String,
    pub volume: f64,
    //ranges from 0 to 1, 0 is full left ear and 1 is full right ear.
    pub panning: f64,
    pub speed: f64,
    pub reversed: bool,
}

impl Default for PlayBGMEvent {
    fn default() -> Self {
        PlayBGMEvent {
            path: String::from("audio/bgm/petscop/explore.mp3"),
            volume: 1.0,
            panning: 0.5,
            speed: 1.0,
            reversed: false,
        }
    }
}

use crate::config::*;

fn play_bgm (
    mut play_bgm_evr: EventReader<PlayBGMEvent>,
    server: Res<AssetServer>,
    config: Res<ConfigFile>,

    audio: Res<AudioChannel<BGM>>,
) {
    for ev in play_bgm_evr.iter() {
        info!("playing the song at path {} /// {:?}", ev.path, ev);

        //scale it by the configured audio scale.
        let volume = ev.volume * config.bgm_volume;

        info!("computed audio for this song is {}", volume);

        //this reversal is jank. i think there's an alternative to the audiobuilder we can use here.
        if ev.reversed {
            audio
                .play(server.load(ev.path.clone()))
                .with_volume(volume)
                .with_panning(ev.panning)
                .with_playback_rate(ev.speed)
                .reverse()
                ;
        }
        else {
            audio
                .play(server.load(ev.path.clone()))
                .with_volume(ev.volume)
                .with_panning(ev.panning)
                .with_playback_rate(ev.speed)
                ;
        }
    }
}

pub struct PlaySFXEvent {
    path: String,
}

fn play_sfx(
    mut play_sfx_evr: EventReader<PlaySFXEvent>,
    server: Res<AssetServer>,

    audio: Res<AudioChannel<SFX>>,
) {
}

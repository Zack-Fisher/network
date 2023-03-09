use crate::file_system_interaction::asset_loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::{Audio, *};

/// Handles initialization of all sounds.
pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlaySong>()
            .add_event::<PlaySFX>()

            .add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(init_audio))
            ;
    }
}

#[derive(Debug, Clone, Resource)]
pub struct AudioHandles {
    pub walking: Handle<AudioInstance>,
}

pub struct PlaySong {
    channel: SongChannels,
}

pub struct PlaySFX {
    channel: SFXChannels,
}

fn init_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.walking.clone())
        .looped()
        .with_volume(0.8)
        .handle();
    commands.insert_resource(AudioHandles { walking: handle });
}

fn play_songs (
    mut song_evr: EventReader<PlaySong>,

    main: Res<AudioChannel<MainChannel>>,
    background: Res<AudioChannel<BackgroundChannel>>,
)
{


    for ev in song_evr.iter() {
        match ev.channel {
            SongChannels::Main => {
                
            }
            SongChannels::Background => {

            }
        }
    }
}

fn play_sfx (
    mut sfx_evr: EventReader<PlaySFX>,
)
{
    for ev in sfx_evr.iter() {

    }
}

#[derive(Resource)]
struct MainChannel;

#[derive(Resource)]
struct BackgroundChannel;

pub enum SongChannels {
    Main,
    Background,
}

struct PlayerSFXChannel;

pub enum SFXChannels {
    Player,
}

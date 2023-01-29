use bevy::prelude::*;

pub struct BGMPlugin;

impl Plugin for BGMPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(play_bgm);
    }
}

fn play_bgm(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    //resolve the pointer with the asset server, then play it!
    let bgm = asset_server.load("audio/bgm/petscop/explore.mp3");
    audio.play(bgm);
}

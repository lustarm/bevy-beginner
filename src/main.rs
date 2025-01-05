mod game;
mod assets;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

fn main() {
    App::new()
    .init_state::<game::State>()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_loading_state(
        LoadingState::new(game::State::Loading)
        .continue_to_state(game::State::Main)
    )
    .run();
}


mod game;
mod assets;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .init_collection::<assets::PlayerAssets>()
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset: Res<assets::PlayerAssets>) {
    commands.spawn(Camera2d{..default()});
}
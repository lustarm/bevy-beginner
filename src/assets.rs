use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 8, rows = 1, padding_x = 12, padding_y = 12, offset_x = 6, offset_y = 6))]
    #[asset(path = "images/Idle.png")]
    sprite: Handle<Image>
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub frame_count: usize,
}

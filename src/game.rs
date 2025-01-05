use bevy::prelude::*;

use crate::assets;

#[derive(Default, PartialEq, Eq, Debug, Hash, States, Clone)]
pub enum State {
    #[default]
    Loading,
    Main,
}

fn spawn_player(mut commands: Commands, asset: Res<assets::PlayerAssets>) {
    let sprite = Sprite {
        custom_size: Some(Vec2::splat(140.0)),
        ..default()
    };

    commands.spawn((
        assets::AnimationTimer {
            timer: Timer::from_seconds(0.125, TimerMode::Repeating),
            frame_count: 11
        },
    ));
}
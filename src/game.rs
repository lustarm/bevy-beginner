use bevy::prelude::States;

#[derive(Default, PartialEq, Eq, Debug, Hash, States, Clone)]
pub enum State {
    #[default]
    Loading,
    Main,
}
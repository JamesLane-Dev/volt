use bevy::prelude::*;

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    MainMenu,
    #[default]
    Playing,
    GameOver,
}

mod state;
mod ui;
mod volt;
mod world;
mod enemy;

use crate::state::GameState;
use crate::ui::UiPlugin;
use crate::volt::VoltPlugin;
use crate::world::WorldPlugin;
use crate::enemy::EnemyPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(VoltPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

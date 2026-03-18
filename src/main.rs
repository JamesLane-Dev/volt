mod state;
mod ui;
mod volt;
mod world;

use crate::state::GameState;
use crate::ui::UiPlugin;
use crate::volt::VoltPlugin;
use crate::world::WorldPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(VoltPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(WorldPlugin)
        .run();
}

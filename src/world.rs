use crate::volt::Volt;
use bevy::prelude::*;

const GRID_SIZE: u32 = 64;
const TILE_SIZE: f32 = 32.0;
const VISION_RANGE: f32 = 256.0;
#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub tile_visible: TileVisibility,
}
pub enum TileType {
    Grass,
    Water,
    Bush,
    Land,
}
#[derive(PartialEq, Eq)]
pub enum TileVisibility {
    Hidden,
    Revealed,
    Visible,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, fog_of_war)
            .add_systems(Update, update_tile_visibility);
    }
}
fn setup(mut commands: Commands) {
    for i in 0..GRID_SIZE {
        for n in 0..GRID_SIZE {
            let x = i as f32 * TILE_SIZE - (GRID_SIZE as f32 * TILE_SIZE / 2.0);
            let y = n as f32 * TILE_SIZE - (GRID_SIZE as f32 * TILE_SIZE / 2.0);

            commands.spawn((
                Tile {
                    tile_type: TileType::Grass,
                    tile_visible: TileVisibility::Hidden,
                },
                Sprite {
                    color: Color::srgb(0.6, 0.8, 0.6),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, -2.0),
            ));
        }
    }
}
fn fog_of_war(
    player_query: Query<&Transform, (With<Volt>, Without<Tile>)>,
    mut query: Query<(&Transform, &mut Tile), (With<Tile>, Without<Volt>)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    for (transform, mut tile) in query.iter_mut() {
        let distance = player.translation.distance(transform.translation);

        if distance <= VISION_RANGE {
            tile.tile_visible = TileVisibility::Visible;
        } else if tile.tile_visible == TileVisibility::Visible {
            tile.tile_visible = TileVisibility::Revealed;
        }
    }
}
fn update_tile_visibility(mut tile_query: Query<(&Tile, &mut Sprite)>) {
    for (tile, mut sprite) in tile_query.iter_mut() {
        if tile.tile_visible == TileVisibility::Hidden {
            sprite.color = Color::srgb(0.0, 0.0, 0.0);
        }
        if tile.tile_visible == TileVisibility::Visible {
            sprite.color = Color::srgb(0.6, 0.8, 0.6);
        }
        if tile.tile_visible == TileVisibility::Revealed {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
}

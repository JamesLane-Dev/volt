use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::state::GameState;

use crate::volt;
use crate::volt::Energy;
use crate::volt::Volt;

// --- Const ---
const BAR_WIDTH: f32 = 60.0;
const BAR_HEIGHT: f32 = 10.0;

#[derive(Component)]
pub struct EnergyBar;
#[derive(Component)]
pub struct RedBar;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.after(volt::setup))
            .add_systems(
                Update,
                update_energy_bar.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, follow_volt.run_if(in_state(GameState::Playing)));
    }
}
fn setup(mut commands: Commands, energy_query: Query<&Energy, With<Volt>>) {
    let Ok(energy) = energy_query.single() else {
        return;
    };
    let width = energy.current / energy.max * BAR_WIDTH;
    commands.spawn((
        EnergyBar,
        Anchor::CENTER_LEFT,
        Sprite {
            color: Color::srgb(0.3, 1.0, 0.3),
            custom_size: Some(Vec2::new(width, BAR_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-BAR_WIDTH / 2.0, 45.0, -1.0),
    ));
    commands.spawn((
        RedBar,
        Anchor::CENTER_LEFT,
        Sprite {
            color: Color::srgb(1.0, 0.3, 0.3),
            custom_size: Some(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-BAR_WIDTH / 2.0, 45.0, -1.0),
    ));
}
fn update_energy_bar(
    player_query: Query<&Energy, With<Volt>>,
    mut bar_query: Query<&mut Sprite, With<EnergyBar>>,
) {
    let Ok(energy) = player_query.single() else {
        return;
    };
    let Ok(mut sprite) = bar_query.single_mut() else {
        return;
    };

    let width = energy.current / energy.max * BAR_WIDTH;
    sprite.custom_size = Some(Vec2::new(width, BAR_HEIGHT));
}
fn follow_volt(
    player_query: Query<&Transform, (With<Volt>, Without<EnergyBar>)>,
    mut green_bar_query: Query<&mut Transform, (With<EnergyBar>, Without<Volt>, Without<RedBar>)>,
    mut red_bar_query: Query<&mut Transform, (With<RedBar>, Without<Volt>, Without<EnergyBar>)>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };
    let Ok(mut green_bar) = green_bar_query.single_mut() else {
        return;
    };
    let Ok(mut red_bar) = red_bar_query.single_mut() else {
        return;
    };

    green_bar.translation.x = transform.translation.x + (-BAR_WIDTH / 2.0);
    green_bar.translation.y = transform.translation.y + 45.0;
    red_bar.translation.x = transform.translation.x + (-BAR_WIDTH / 2.0);
    red_bar.translation.y = transform.translation.y + 45.0;
}

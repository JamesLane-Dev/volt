use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct Volt {
    pub speed: f32,
}
#[derive(Component)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}
pub struct VoltPlugin;

impl Plugin for VoltPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, energy_drain.run_if(in_state(GameState::Playing)))
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                follow_player_camera.run_if(in_state(GameState::Playing)),
            );
    }
}

fn energy_drain(time: Res<Time>, mut query: Query<&mut Energy, With<Volt>>) {
    let dt = time.delta_secs();
    let Ok(mut energy) = query.single_mut() else {
        return;
    };
    energy.current -= 0.0833 * dt;
    println!("Energy {}", energy.current);
}
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Volt { speed: 1.0 },
        Energy {
            current: 100.0,
            max: 150.0,
        },
        Sprite {
            color: Color::srgb(0.3, 0.3, 1.0),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // --- Temorary sprite for reference ---
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.6, 0.4),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(35.0, -35.0, 0.0),
    ));
}
fn movement(
    mut query: Query<(&mut Transform, &Volt), With<Volt>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, volt)) = query.single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::KeyD) {
        transform.translation.x += volt.speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        transform.translation.x -= volt.speed;
    }
    if keys.pressed(KeyCode::KeyW) {
        transform.translation.y += volt.speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        transform.translation.y -= volt.speed;
    }
}
fn follow_player_camera(
    player_query: Query<&Transform, (With<Volt>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Volt>)>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    camera.translation.x = transform.translation.x;
    camera.translation.y = transform.translation.y;
}

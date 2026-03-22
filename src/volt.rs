use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::enemy::Enemy;
use crate::state::GameState;

const LASER_SPEED: f32 = 400.;

#[derive(Component)]
pub struct Volt {
    pub speed: f32,
}
#[derive(Component)]
pub struct Laser {
    pub direction: Vec2,
}
#[derive(Component)]
pub struct CrossHair;
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
            .add_systems(Update, move_crosshair.run_if(in_state(GameState::Playing)))
            .add_systems(Update, fire_laser.run_if(in_state(GameState::Playing)))
            .add_systems(Update, despawn_laser.run_if(in_state(GameState::Playing)))
            .add_systems(Update, laser_hit.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_laser.run_if(in_state(GameState::Playing)))
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
        Volt { speed: 125.0 },
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
    commands.spawn((
        CrossHair,
        Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(60.0, 60.0, 0.0),
    ));
}
fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Volt), With<Volt>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, volt)) = query.single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::KeyD) {
        transform.translation.x += volt.speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyA) {
        transform.translation.x -= volt.speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyW) {
        transform.translation.y += volt.speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyS) {
        transform.translation.y -= volt.speed * time.delta_secs();
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
fn move_crosshair(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut crosshair: Query<&mut Transform, With<CrossHair>>,
) {
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((cam, cam_transform)) = camera.single() else {
        return;
    };
    let Ok(world_pos) = cam.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };
    let Ok(mut crosshair) = crosshair.single_mut() else {
        return;
    };
    crosshair.translation.x = world_pos.x;
    crosshair.translation.y = world_pos.y;
}
fn fire_laser(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Volt>>,
    crosshair_query: Query<&Transform, With<CrossHair>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(crosshair) = crosshair_query.single() else {
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let x = crosshair.translation.x - player_transform.translation.x;
    let y = crosshair.translation.y - player_transform.translation.y;
    let direction = Vec2::new(x, y).normalize_or_zero();

    commands.spawn((
        Laser { direction },
        Sprite {
            color: Color::srgb(0.6, 1.0, 0.5),
            custom_size: Some(Vec2::new(6.0, 6.0)),
            ..default()
        },
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y,
            1.0,
        ),
    ));
}
fn move_laser(time: Res<Time>, mut query: Query<(&mut Transform, &Laser), With<Laser>>) {
    for (mut transform, laser) in query.iter_mut() {
        transform.translation.x += laser.direction.x * LASER_SPEED * time.delta_secs();
        transform.translation.y += laser.direction.y * LASER_SPEED * time.delta_secs();
    }
}
fn despawn_laser(
    mut commands: Commands,
    player_query: Query<&Transform, With<Volt>>,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
) {
    let Ok(volt_transform) = player_query.single() else {
        return;
    };
    for (entity, transform) in laser_query.iter() {
        let x = volt_transform.translation.x - transform.translation.x;
        let y = volt_transform.translation.y - transform.translation.y;

        let distance = Vec2::new(x, y).length();
        if distance > 500.0 {
            commands.entity(entity).despawn();
        }
    }
}
fn laser_hit(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform), With<Enemy>>,
) {
    for (laser_entity, laser_transform) in laser_query.iter() {
        for (entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let x = enemy_transform.translation.x - laser_transform.translation.x;
            let y = enemy_transform.translation.y - laser_transform.translation.y;

            let distance = Vec2::new(x, y).length();
            if distance < 20.0 {
                commands.entity(laser_entity).despawn();
                enemy.health -= 10.0;
                if enemy.health <= 0.0 {
                    commands.entity(entity).despawn();
                }
                break;
            }
        }
    }
}

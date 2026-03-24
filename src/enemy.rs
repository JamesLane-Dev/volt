use crate::state::GameState;
use crate::volt::Volt;
use ::rand::Rng;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 100.0;
const ENEMY_COUNT: u32 = 5;
#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}
#[derive(Component, PartialEq, Eq)]
pub enum EnemyState {
    Chase,
    Guard,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_setup)
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    }
}
pub fn enemy_setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _i in 0..ENEMY_COUNT {
        let x = rng.gen_range(-1024.0..1024.0);
        let y = rng.gen_range(-1024.0..1024.0);
        commands.spawn((
            EnemyState::Guard,
            Enemy {
                health: 100.0,
                speed: ENEMY_SPEED,
            },
            Sprite {
                color: Color::srgb(1.0, 0.6, 0.6),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}
pub fn movement(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Volt>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyState), (With<Enemy>, Without<Volt>)>,
) {
    let Ok(volt_transform) = player_query.single() else {
        return;
    };
    for (mut enemy, mut state) in enemy_query.iter_mut() {
        let x = (volt_transform.translation.x - enemy.translation.x);
        let y = (volt_transform.translation.y - enemy.translation.y);

        let direction = Vec2::new(x, y).normalize_or_zero();
        let distance = Vec2::new(x, y).length();
        if distance < 224.0 && *state == EnemyState::Guard {
            *state = EnemyState::Chase;
        }
        if distance > 375.0 && *state == EnemyState::Chase {
            *state = EnemyState::Guard;
        }
        if *state == EnemyState::Chase {
            enemy.translation.x += direction.x * ENEMY_SPEED * time.delta_secs();
            enemy.translation.y += direction.y * ENEMY_SPEED * time.delta_secs();
        }
    }
}

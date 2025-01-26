use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{camera::PanOrbitState, characters::CharacterControllerBundle};

pub struct TruckPlugin;

impl Plugin for TruckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_target);
    }
}

#[derive(Component)]
pub struct Vehicle {}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3,
    rotation: f32,
) {
    let my_scene: Handle<Scene> = server.load("models/placeholder/truck.glb#Scene0");
    commands
        .spawn((
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
            SceneRoot(my_scene),
            Player {},
            Vehicle {},
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(50.0, 0.92, 7.0, (50.0 as Scalar).to_radians()),
        ))
        .with_children(|parent| {
            // child cube
            parent.spawn((
                Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                }),
                Collider::cuboid(6.0, 4.0, 10.0),
            ));
        });
}

pub fn update_camera_target(
    players: Query<&Transform, With<Player>>,
    mut pan_orbits: Query<&mut PanOrbitState>,
) {
    let single_player = players.get_single();
    if single_player.is_err() {
        return;
    }

    let player_transform = single_player.unwrap();

    for mut pan_orbit in &mut pan_orbits {
        pan_orbit.center = player_transform.translation;
    }
}

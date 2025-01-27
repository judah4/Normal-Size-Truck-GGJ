use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::{camera::PanOrbitState, characters::CharacterControllerBundle};

pub struct TruckPlugin;

impl Plugin for TruckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_camera_target, print_collisions, print_started_collisions));
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
            Name::new("Player Truck"),
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
            SceneRoot(my_scene),
            Player {},
            Vehicle {},
            CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(70.0, PI, 0.92, 7.0, (50.0 as Scalar).to_radians()),
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
        pan_orbit.center = player_transform.translation
            + Vec3 {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            };

        let player_rotation = player_transform.rotation.to_axis_angle();
        let axis_angle = player_rotation.0.y * player_rotation.1;
        //let half_life = half_life_from_precision(1.0, 0.1);
        //let lerp = lerp_smooth(
        //    pan_orbit.yaw,
        //    axis_angle,
        //    time.delta_secs(),
        //    half_life,
        //);
        //debug!("axis:{}, {}, yaw:{}", player_rotation.0, axis_angle, pan_orbit.yaw);
        pan_orbit.yaw = axis_angle;
    }
}

/// Freya's notes listing equations to do framerate independent lerp-smoothing.
/// The main formula is given by B+(A minus B) times 2 to the power of negative delta time divided by the half-life  
/// https://mastodon.social/@acegikmo/111931613710775864  
pub fn lerp_smooth(a: f32, b: f32, dt: f32, half_life: f32) -> f32 {
    let lerp = b + (a - b) * f32::exp2(-dt / half_life);
    lerp
}

/// Calculating half-life given a duration until precision.
///
/// # Examples
///
/// ```
/// use crate::trucks::half_life_from_precision;
///
/// assert_eq!(half_life_from_precision(1.0, 0.01), 0.150515);
/// ```
pub fn half_life_from_precision(duration: f32, precision: f32) -> f32 {
    let half_life = -duration / f32::log2(precision);
    half_life
}

fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        if !contacts.is_sensor {
            return;
        }

        // TODO: Get when truck enters trigger / sensor

        info!(
            "Entities {} and {} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}

fn print_started_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        debug!(
            "Entities {} and {} started colliding",
            entity1,
            entity2,
        );
    }
}

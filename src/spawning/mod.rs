use avian3d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct MilkSpot;

#[derive(Component)]
pub struct HomeSpot;

pub fn load_world(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 15_000.0,
            shadows_enabled: true,
            ..default()
        },
    ));

    commands.spawn((
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: -0.5,
            z: 0.0,
        }),
        RigidBody::Static,
        Collider::cuboid(4000.0, 1.0, 4000.0),
    ));

    commands.spawn((
        Transform::from_translation(Vec3 {
            x: 50.0,
            y: 0.0,
            z: 100.0,
        }),
        RigidBody::Static,
        Collider::sphere(10.0),
        Sensor,
        MilkSpot,
        Name::new("Milk Spot"),
    ));

    commands.spawn((
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
        RigidBody::Static,
        Collider::sphere(10.0),
        Sensor,
        HomeSpot,
        Name::new("Home Spot"),
    ));

    load_truck(
        &mut commands,
        &server,
        Vec3 {
            x: 10.0,
            y: 0.0,
            z: 10.0,
        },
        PI * 0.5,
    );
    load_person(
        &mut commands,
        &server,
        Vec3 {
            x: 5.0,
            y: 0.0,
            z: -1.0,
        },
    );
    load_default_cube(&mut commands, &server);

    let mut index = 0;
    let tile_size = 4.0;
    for x in -100..100 {
        for z in -100..100 {
            if index % 2 == 0 {
                load_plane_road(
                    &mut commands,
                    &server,
                    Vec3 {
                        x: x as f32 * tile_size,
                        y: 0.0,
                        z: z as f32 * tile_size,
                    },
                );
            } else {
                load_plane_grass(
                    &mut commands,
                    &server,
                    Vec3 {
                        x: x as f32 * tile_size,
                        y: 0.0,
                        z: z as f32 * tile_size,
                    },
                );
            }

            if (x + 5) % 10 == 0 && (z + 5) % 10 == 0 {
                load_house(
                    &mut commands,
                    &server,
                    Vec3 {
                        x: x as f32 * tile_size,
                        y: 0.0,
                        z: z as f32 * tile_size,
                    },
                    PI * index as f32 * 0.5,
                );
            }

            index += 1;
        }
        index += 1;
    }
}

fn load_default_cube(commands: &mut Commands, server: &Res<AssetServer>) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/defaultCube.glb#Scene0");
    commands.spawn(SceneRoot(my_scene));
}

fn load_person(commands: &mut Commands, server: &Res<AssetServer>, translation: Vec3) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/person.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation),
        SceneRoot(my_scene),
    ));
}

fn load_house(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3,
    rotation: f32,
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/house.glb#Scene0");
    commands
        .spawn((
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
            SceneRoot(my_scene),
        ))
        .with_children(|parent| {
            // child cube
            parent.spawn((
                Transform::from_translation(Vec3 {
                    x: 0.0,
                    y: 5.0,
                    z: 0.0,
                }),
                RigidBody::Static,
                Collider::cuboid(10.0, 10.0, 10.0),
            ));
        });
}

fn load_truck(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3,
    rotation: f32,
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/truck.glb#Scene0");
    commands
        .spawn((
            Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
            SceneRoot(my_scene),
            RigidBody::Dynamic,
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

fn load_plane_grass(commands: &mut Commands, server: &Res<AssetServer>, translation: Vec3) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/plane_grass.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation),
        SceneRoot(my_scene),
    ));
}

fn load_plane_road(commands: &mut Commands, server: &Res<AssetServer>, translation: Vec3) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/plane_road.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation),
        SceneRoot(my_scene),
    ));
}

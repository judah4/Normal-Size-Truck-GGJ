use std::f32::consts::PI;

use bevy::prelude::*;

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

    load_truck(&mut commands, &server, Vec3::ZERO, 0.0);
    load_person(&mut commands, &server, Vec3{x: 5.0, y: 0.0, z: -1.0});
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
                    PI * index as f32,
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

fn load_person(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3,
) {
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
    commands.spawn((
        Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
        SceneRoot(my_scene),
    ));
}

fn load_truck(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3,
    rotation: f32,
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/truck.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation).with_rotation(Quat::from_rotation_y(rotation)),
        SceneRoot(my_scene),
    ));
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

fn load_default_plane(commands: &mut Commands, server: &Res<AssetServer>, translation: Vec3) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/defaultPlane.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation),
        SceneRoot(my_scene),
    ));
}

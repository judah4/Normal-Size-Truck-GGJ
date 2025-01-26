use bevy::prelude::*;

pub fn load_world( mut commands: Commands,
    server: Res<AssetServer>) {

        commands.spawn((
            Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            DirectionalLight {
                color: Color::WHITE,
                illuminance: 15_000.0,
                shadows_enabled: true,
                ..default()
        }));

    load_default_cube(&mut commands, &server);

    let tile_size = 4.0;
    for x in -10..10 {
        for z in -10..10 {
            load_default_plane(&mut commands, &server, Vec3{x: x as f32 * tile_size, y: 0.0, z: z as f32 * tile_size});
        }
    }

}

fn load_default_cube(
    commands: &mut Commands,
    server: &Res<AssetServer>
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/defaultCube.glb#Scene0");
    commands.spawn(SceneRoot(my_scene));
}

fn load_default_plane(
    commands: &mut Commands,
    server: &Res<AssetServer>,
    translation: Vec3
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/placeholder/defaultPlane.glb#Scene0");
    commands.spawn((
        Transform::from_translation(translation),
        SceneRoot(my_scene)));
}
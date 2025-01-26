use avian3d::prelude::*;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    window::PresentMode,
    winit::WinitSettings,
};
use camera::TruckCameraPlugin;
use spawning::load_world;

mod camera;
mod spawning;
mod trucks;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.3921, 0.5843, 0.9294))) //Cornflower Blue
        .insert_resource(WinitSettings::game())
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Normal Size Truck".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: bevy::render::settings::RenderCreation::Automatic(
                        WgpuSettings {
                            // Try to force compatibility so that it uses Vulkan.
                            //backends: Some(Backends::VULKAN),
                            priority: bevy::render::settings::WgpuSettingsPriority::Compatibility,
                            ..default()
                        },
                    ),
                    ..default()
                }),
            PhysicsPlugins::default(),
        ))

        // Debug systems
        // Enables debug rendering
        .add_plugins(PhysicsDebugPlugin::default())

        // Add game systems
        .add_plugins(TruckCameraPlugin)
        .add_systems(Startup, load_world)
        .run();
}

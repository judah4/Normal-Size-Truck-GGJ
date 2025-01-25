
use bevy::{prelude::*, render::{settings::WgpuSettings, RenderPlugin}, window::PresentMode, winit::WinitSettings};

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
        ))
        // add the app state type
        .run();
}
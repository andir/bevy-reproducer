use bevy::app::ScheduleRunnerSettings;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::Duration;

mod cube;
mod light;

struct Person;
struct Name(String);

fn add_camera(commands: &mut Commands) {
    commands.spawn(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
            .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
    });
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_camera.system());
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "AM".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(HelloPlugin)
        .add_plugin(cube::CubePlugin)
        .add_plugin(light::LightPlugin)
        .run();
}

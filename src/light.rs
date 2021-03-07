use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(move_light.system());
    }
}

struct Light;

pub fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .with(Light);
}

/// Rotate the light by 10 degrees on the x-z-plane every second, full rotation taking 36s
fn move_light(time: Res<Time>, mut query: Query<&mut Transform, With<Light>>) {
    let d = time.delta_seconds();
    let angle = 10.0_f32.to_radians();
    let delta_angle = (angle * d) % angle;
    for mut light in query.iter_mut() {
        light.translation = Vec3::new(
            light.translation.x * delta_angle.cos() - light.translation.z * delta_angle.sin(),
            light.translation.y,
            light.translation.x * delta_angle.sin() + light.translation.z * delta_angle.cos(),
        );
    }
}

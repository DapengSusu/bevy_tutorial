use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "bevy_tuturial".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera_system)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera_system(mut command: Commands) {
    let graph_projection = OrthographicProjection {
        bottom: -1.0,
        top: 1.0,
        left: -1.0 * RESOLUTION,
        right: 1.0 * RESOLUTION,
        scaling_mode: ScalingMode::None,
        ..Default::default()
    };

    command.spawn_bundle(Camera2dBundle {
        projection: graph_projection,
        ..Default::default()
    });
}

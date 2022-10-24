use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 480.0,
            height: 320.0,
            title: "bevy_tuturial".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}

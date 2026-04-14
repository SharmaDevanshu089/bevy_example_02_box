use bevy::prelude::*;

fn main() {
    //Main Loader for bevy
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, SpawnCamera)
        .run();
}

fn SpawnCamera(mut commands: Commands) {
    // This will spawn camera at 1000 z axis
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 1000.0)));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::hsl(210.0, 0.4, 0.3),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

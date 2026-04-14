use bevy::{prelude::*, sprite};

fn main() {
    //Main Loader for bevy
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, SpawnCamera)
        .run();
}

fn SpawnCamera(mut commands: Commands) {
    // This will spawn camera at 1000 z axis
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
        GlobalTransform::default(),
    ));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::hsl(0.2, 0.3, 0.7),
            custom_size: Some(Vec2::new(120.0, 80.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

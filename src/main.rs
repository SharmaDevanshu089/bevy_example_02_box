use bevy::prelude::*;

#[derive(Component)]
struct SubjectBox;

fn main() {
    //Main Loader for bevy
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, SpawnCamera)
        .add_systems(Update, change_color_after_ten_seconds)
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
        SubjectBox,
    ));
}

fn change_color_after_ten_seconds(
    time: Res<Time>,
    mut query: Query<&mut Sprite, With<SubjectBox>>,
) {
    if time.elapsed_secs() > 10.0 {
        println!("time has been changed");
    }
}

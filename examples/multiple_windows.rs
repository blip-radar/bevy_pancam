use bevy::{camera::RenderTarget, prelude::*, window::WindowRef};
use bevy_pancam::{PanCam, PanCamPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // First window: camera renders to the default primary window.
    commands.spawn((Camera2d, PanCam::default()));

    // Spawn a second OS window.
    let second_window = commands
        .spawn(Window {
            title: "Second window — independent pan/zoom".to_owned(),
            ..default()
        })
        .id();

    // Second camera renders to the second window.
    commands.spawn((
        Camera2d,
        PanCam::default(),
        RenderTarget::Window(WindowRef::Entity(second_window)),
    ));

    // Spawn some sprites so there is something to pan/zoom over.
    let colors = [
        Color::hsl(0., 0.6, 0.5),
        Color::hsl(120., 0.6, 0.5),
        Color::hsl(240., 0.6, 0.5),
    ];
    for (i, color) in colors.iter().enumerate() {
        let x = (i as f32 - 1.) * 200.;
        commands.spawn((
            Sprite {
                color: *color,
                custom_size: Some(Vec2::splat(150.)),
                ..default()
            },
            Transform::from_xyz(x, 0., 0.),
        ));
    }
}

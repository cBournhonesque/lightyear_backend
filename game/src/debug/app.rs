//! Debug app to display something on the screen
use bevy::prelude::*;


pub(crate) struct DebugAddPlugin;

impl Plugin for DebugAddPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

/// Init a camera and some debug text
fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "Hello World",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ),
    ));
}
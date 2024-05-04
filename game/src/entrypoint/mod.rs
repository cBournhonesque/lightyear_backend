use bevy::prelude::*;

/// Create the app and start the game.
///
/// `f` can be used to further modify the app.
pub fn start(f: impl FnOnce(&mut App)) {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    ..default()
                }),
                ..default()
            })
    );

    f(&mut app);

    app.run();
}
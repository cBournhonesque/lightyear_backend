use debug::DebugAddPlugin;
use entrypoint::start;

fn main() {
    start(|app| {
        app.add_plugins(DebugAddPlugin);
    });
}
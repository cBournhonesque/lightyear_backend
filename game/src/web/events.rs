//! Module to receive messages from the leptos app
use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use once_cell::sync::OnceCell;


pub struct EventStreamPlugin;

static EVENT_CHANNEL: OnceCell<(crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>)> = OnceCell::new();

impl Plugin for EventStreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_event_stream)
            .add_systems(Update, read_event_stream);
    }
}

/// Channel to receive events to the website
#[derive(Resource)]
pub struct Channel(crossbeam_channel::Receiver<()>);

fn register_event_stream(mut commands: Commands) {
    EVENT_CHANNEL.set(crossbeam_channel::unbounded::<()>()).unwrap();
    commands.insert_resource(Channel(EVENT_CHANNEL.get().unwrap().1.clone()));
}

fn read_event_stream(channel: Res<Channel>) {
    while channel.0.try_recv().is_ok() {
        println!("Received event on channel");
    }
}

/// Function that will use in the leptos website to send events to bevy
#[wasm_bindgen(js_name = "sendGameEvent")]
pub fn send_game_event() -> Result<(), JsError> {
    EVENT_CHANNEL.get().unwrap().0.send(()).map_err(Into::into)
}
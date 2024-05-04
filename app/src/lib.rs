use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::spawn::Spawn;

pub(crate) mod components;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/lightyear_backend.css"/>

        // will store events that we want to send to the game
        // - globalThis['innerSendGameEvent'] is a function from the game's wasm to send an event to the game
        <script>
            "
            let pendingEvents = [];
            function sendGameEvent(event) {
                if (typeof globalThis['innerSendGameEvent'] !== 'function') {
                    pendingEvents.push(event);
                } else {
                    globalThis['innerSendGameEvent'](event);
                }
            }
            "
        </script>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/lobbies" view=HomePage/>
                    <Route path="/game" view=components::game::Game/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Spawn/>
    }
}
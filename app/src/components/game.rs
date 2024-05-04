use leptos::*;

/// Server function that starts the game using the connect token
/// 1. Compile game with wasm_bindgen
/// 2. Create a DOM element
pub async fn launch_game() {

}

// TODO: We want to
//  1. start the app
//  2. have a way to send messages into the app.
//  3. we send a message saying StartGame that contains the NetworkToken
//  4. the game receives that message, and tries to connect!
#[component]
pub(crate) fn game() -> impl IntoView {
    view! {
        <script type="module">
            "
            import init, { sendGameEvent as innerSendGameEvent } from '/game.js';

            init().catch(() => {
                window['innerSendGameEvent'] = innerSendGameEvent;
                console.log('Module initialised, flushing pending events')
                while (pendingEvents.length > 0) {
                    let event = pendingEvents.shift();
                    innerSendGameEvent(event);
                }
            });
            "
        </script>
    }
}


// #[component]
// pub(crate) fn game() -> impl IntoView {
//     let document = leptos::document();
//     document.create_element("script");
//     view! {
//         <canvas id="bevy" class="pointer-events-auto absolute top-0 left-0 -z-10"></canvas>
//         <script type="module">
//             import init, { run } from "/simple_box.js";
//             async function run() {
//                 // First up we need to actually load the wasm file, so we use the
//                 // default export to inform it where the wasm file is located on the
//                 // server, and then we wait on the returned promise to wait for the
//                 // wasm to be loaded.
//                 await init();
//
//                 // And afterwards we can use all the functionality defined in wasm.
//                 run(1, 2);
//             }
//             run();
//         </script>
//     }
// }
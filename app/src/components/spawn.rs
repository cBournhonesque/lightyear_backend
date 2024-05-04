use leptos::*;
use leptos::ev::SubmitEvent;
use bytes::Bytes;
use leptos_router::{ActionForm};

/// Server Function that returns a connect token so that the client can connect to the server
/// 2. Call the matchmaker service to find which server is available and get the server's address
/// 3. Generate the ConnectToken
#[server(GetConnectToken)]
pub async fn get_connect_token(name: String) -> Result<Bytes, ServerFnError> {
    use lightyear::connection::netcode::ConnectToken;
    use lightyear::prelude::*;
    use crate::components::edgegap::{start_session};
    pub const PROTOCOL_ID: u64 = 0;
    pub const KEY: Key = [0; 32];

    // 1. Generate a unique client id that is not currently used (maintain global state)
    let client_id = 1;
    let client_ip = "68.173.153.4".to_string();

    let session_data = start_session(client_ip).await?;

    // wait for deployment to be ready
    let deployment = session_data.deployment.ok_or(ServerFnError::new("No deployment available"))?;
    let server_ip = deployment.public_ip;
    let server_port = deployment.ports
        .map(|m| m.get("Game").map(|x| x.external.unwrap()))
        .flatten()
        .ok_or(
            ServerFnError::new("No port found")
        )?;
    let server_addr = format!("{}:{}", server_ip, server_port);

    // 3. Generate the ConnectToken
    let token = ConnectToken::build(server_addr, PROTOCOL_ID, client_id, KEY)
        .expire_seconds(60)
        .timeout_seconds(30)
        .generate()?;
    let token_bytes = token.try_into_bytes()?.to_vec();
    Ok(Bytes::from(token_bytes))
}


#[component]
pub(crate) fn Spawn() -> impl IntoView {
    let token = create_server_action::<GetConnectToken>();
    // let once = create_resource(|| (), |_| async { get_connect_token().await });
    let on_submit = move |ev: SubmitEvent| {
        // let data = GetConnectToken::from_event(ev);
        // let data = token.value().get();
        // console_log(&format!("data: {:?}", data));
        // stop the page from reloading!
        ev.prevent_default();
        // let once = create_resource(|| (), |_| async { get_connect_token().await });
    };
    view! {
        <ActionForm
            action=token
            class="flex items-center sm:col-span-10 border-b border-gray-900/10 pb-12"
        >
            <label class="block text-sm font-medium leading-6 text-gray-900">
                "Player Name"
                <input
                    type="text"
                    class="block flex-1 border-0 py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                    name="name"
                />
            </label>
            // <input type="submit" id="button" value="Play"/>
            <button class="rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
                "Play"
            </button>
        </ActionForm>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>
                <p class="rounded-xl flex items-center">
                    The result was:
                    {move || {
                        token
                            .value()
                            .get()
                            .map(|b| b.map(|token| format!("{:?}", token)))
                    }}
                </p>
            </ErrorBoundary>
        </Suspense>
    }
}
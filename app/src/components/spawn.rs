use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use leptos::*;
use leptos::ev::SubmitEvent;
use leptos::leptos_dom::logging::console_log;
use leptos_router::{ActionForm, FromFormData};
use lightyear::netcode::ConnectToken;
use lightyear::prelude::*;
use edgegap::apis::applications_api::app_versions_patch;
use edgegap::apis::configuration::{ApiKey, Configuration};
use edgegap::apis::deployments_api::deployments_get;
use edgegap::models::AppVersionUpdatePayload;
use crate::components::edgegap::start_deployment;

pub const PROTOCOL_ID: u64 = 0;
pub const KEY: Key = [0; 32];


/// Server Function that returns a connect token so that the client can connect to the server
/// 2. Call the matchmaker service to find which server is available and get the server's address
/// 3. Generate the ConnectToken
#[server(GetConnectToken)]
pub async fn get_connect_token(name: String) -> Result<String, ServerFnError> {
    // 1. Generate a unique client id that is not currently used
    // let client_id = 1;
    let client_ip = "68.173.153.4".to_string();

    start_deployment(client_ip).await?;

    // wait for deployment to be ready
    // call context API to get the port-mapping
    // create context token

    // 2.b Call the matchmaker service to find which server is available and get the server's address
    // let server_addr = "212.2.246.18:4000";

    // 3. Generate the ConnectToken
    // let token = ConnectToken::build(server_addr, PROTOCOL_ID, client_id, KEY)
    //     .expire_seconds(60)
    //     .timeout_seconds(30)
    //     .generate()?;
    // let token = server_data["token"].as_str().unwrap();

    // let bytes = Bytes::from("hello");
    Ok(String::from(format!("Name is {}", name)))
}


#[component]
pub(crate) fn Spawn() -> impl IntoView {
    // let styler_class = style! {"Spawn",
    //     r#"form{
    //         display: flex;
    //         flex-direction: column;
    //         position: absolute;
    //         row-gap: 2rem;
    //         user-select: none;
    //         min-width: 50%;
    //         animation: fadein 1s;
    //
    //         @keyframes fadein {
    //             from { opacity: 0; }
    //             to   { opacity: 1; }
    //         }
    //     }
    //     .input {
    //         border-radius: 3rem;
    //         border: 0;
    //         box-sizing: border-box;
    //         color: #FFFA;
    //         cursor: pointer;
    //         font-size: "1.7rem";
    //         font-weight: bold;
    //         margin-top: "0.25em";
    //         outline: 0;
    //         padding-left: 2rem;
    //         padding: "0.7em";
    //         pointer-events: all;
    //         text-align: center;
    //         white-space: nowrap;
    //         width: 100%;
    //     }
    //     #button {
    //         background-color: #549f57;
    //         border-radius: 1rem;
    //         border: 1px solid #61b365;
    //         box-sizing: border-box;
    //         color: white;
    //         cursor: pointer;
    //         font-size: "3.25rem";
    //         left: 50%;
    //         margin-top: "0.5em";
    //         min-width: 12rem;
    //         padding-bottom: "0.7rem";
    //         padding-top: "0.5rem";
    //         position: relative;
    //         text-decoration: none;
    //         transform: translate(-50%, 0%);
    //         white-space: nowrap;
    //         width: min-content;
    //
    //         :disabled {
    //             filter: brightness(0.8);
    //             cursor: initial;
    //         }
    //
    //         :hover:not(:disabled) {
    //             filter: brightness(0.95);
    //         }
    //
    //         :active:not(:disabled) {
    //             filter: brightness(0.9);
    //         }
    //     }"#
    // };
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
        <ActionForm action=token class="flex items-center sm:col-span-10 border-b border-gray-900/10 pb-12">
            <label class="block text-sm font-medium leading-6 text-gray-900">"Player Name"
                <input type="text" class="block flex-1 border-0 bg-transparent py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6" name="name"/>
            </label>
            // <input type="submit" id="button" value="Play"/>
            <button class="rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"> "Play" </button>
        </ActionForm>
        <ErrorBoundary
            fallback=|errors| view! {
                <div class="error">
                    <p>"Errors: "</p>
                    // we can render a list of errors as strings, if we'd like
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                            .collect_view()
                        }
                    </ul>
                </div>
            }
        >
        <p class="rounded-xl flex items-center">The result was: {move || token.value().get()}</p>
        </ErrorBoundary>
        <div class="relative isolate overflow-hidden bg-white px-6 py-24 sm:py-32 lg:px-8">
          <div class="absolute inset-0 -z-10 bg-[radial-gradient(45rem_50rem_at_top,theme(colors.indigo.100),white)] opacity-20"></div>
          <div class="absolute inset-y-0 right-1/2 -z-10 mr-16 w-[200%] origin-bottom-left skew-x-[-30deg] bg-white shadow-xl shadow-indigo-600/10 ring-1 ring-indigo-50 sm:mr-28 lg:mr-0 xl:mr-16 xl:origin-center"></div>
          <div class="mx-auto max-w-2xl lg:max-w-4xl">
            <figure class="mt-10">
              <blockquote class="text-center text-xl font-semibold leading-8 text-gray-900 sm:text-2xl sm:leading-9">
                <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Nemo expedita voluptas culpa sapiente alias molestiae. Numquam corrupti in laborum sed rerum et corporis.</p>
              </blockquote>
              <figcaption class="mt-10">
                <div class="mt-4 flex items-center justify-center space-x-3 text-base">
                  <div class="font-semibold text-gray-900">Judith Black</div>
                  <div class="text-gray-600">CEO of Workcation</div>
                </div>
              </figcaption>
            </figure>
          </div>
        </div>





    }
}
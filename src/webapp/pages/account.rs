use dioxus::prelude::*;

#[cfg(feature = "server")]
use tracing::{debug, info, Level};

use crate::webapp::Route;

#[component]
pub fn Account() -> Element {
    let hometitle = "My Account";
    let hometext = "placeholder for now";
    let homebtn = "Save";

    rsx! {
        div { 
            class: "hero-content flex-col lg:flex-row-reverse",
            div {
                class: "card lg:card-side lg:w-5/6 bg-base-500 shadow-xl text-white",
                div {
                    class: "card-body", 
                    h1 {
                        class: "justify-center text-5xl card-title font-bold",
                        {hometitle}
                    }
                    p {
                        class: "text-2xl py-6",
                        {hometext}
                    }
                    div {
                        class: "card-actions justify-center",
                        Link {
                            class: "btn btn-primary text-xl",
                            to: Route::Login {}, {homebtn} } 
                    }
                }
            }
            }
        }
    }

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

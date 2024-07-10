use dioxus::prelude::*;

#[cfg(feature = "server")]
use tracing::{debug, info, Level};

use crate::webapp::Route;

#[component]
pub fn Home() -> Element {
    let hometitle = "It's your World! Your reality!";
    let hometext = "Create wonderful new campaigns and enjoy DnD with your friends just the way you wanted!";
    let homebtn = "Join Now!";

    rsx! {
        div {
            class: "hero h-full",
            div { 
                class: "hero-content",
                div {
                    class: "card lg:card-side lg:w-5/6 glass shadow-xl text-white",
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
                    figure {
                        img {
                            alt: "Movie",
                            src: "https://i.pinimg.com/originals/d5/98/46/d59846b06d0dd2a415c07af101aaf055.png"
                        }
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

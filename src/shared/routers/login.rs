use dioxus::prelude::*;
use tracing::{debug, info, Level};


#[component]
pub fn Login() -> Element {
    let hometitle = "It's your World! Your reality!";
    let hometext = "Create wonderful new campaigns and enjoy DnD with your friends just the way you wanted!";
    let homebtn = "Join Now!";

    rsx! {
        div { class: "flex justify-center self-center z-10",
        div { class: "p-12 bg-white mx-auto rounded-2xl w-100",
            div { class: "mb-4",
                h3 { class: "font-semibold text-2xl text-gray-800", "Sign In " }
                p { class: "text-gray-500", "Please sign in to your account." }
            }
            div { class: "space-y-5",
                div { class: "space-y-2",
                    label { class: "text-sm font-medium text-gray-700 tracking-wide",
                        "Email"
                    }
                    input {
                        placeholder: "mail@gmail.com",
                        r#type: "",
                        class: "w-full text-base px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:border-green-400"
                    }
                }
                div { class: "space-y-2",
                    label { class: "mb-5 text-sm font-medium text-gray-700 tracking-wide",
                        "Password"
                    }
                    input {
                        r#type: "",
                        placeholder: "Enter your password",
                        class: "w-full content-center text-base px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:border-green-400"
                    }
                }
                div { class: "flex items-center justify-between",
                    div { class: "flex items-center",
                        input {
                            r#type: "checkbox",
                            name: "remember_me",
                            class: "h-4 w-4 bg-blue-500 focus:ring-blue-400 border-gray-300 rounded",
                            id: "remember_me"
                        }
                        label {
                            r#for: "remember_me",
                            class: "ml-2 block text-sm text-gray-800",
                            "Remember me"
                        }
                    }
                    div { class: "text-sm",
                        a {
                            href: "#",
                            class: "text-green-400 hover:text-green-500",
                            "Forgot your password?"
                        }
                    }
                }
                div {
                    button {
                        r#type: "submit",
                        class: "w-full flex justify-center bg-green-400 hover:bg-green-500 text-gray-100 p-3 rounded-full tracking-wide font-semibold shadow-lg cursor-pointer transition ease-in duration-500",
                        "Sign in"
                    }
                }
            }
            div { class: "pt-5 text-center text-gray-400 text-xs",
                span {
                    "Copyright © 2021-2022"
                    a {
                        target: "_blank",
                        rel: "",
                        title: "Ajimon",
                        href: "https://codepen.io/uidesignhub",
                        class: "text-green hover:text-green-500",
                        "AJI"
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

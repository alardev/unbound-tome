use std::{collections::HashMap, error, vec};
use crate::webapp::Route;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use tracing::{debug, info, Level};

use crate::webapp::{model::auth_model::AuthModel, service::validator_service::ValidatorService, APP_STATE};

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    let mut credentials_submit = use_signal(HashMap::<String, FormValue>::new);
    let mut is_busy = use_signal(|| false);

    let is_login_valid = use_memo(move || {
        credentials_submit.is_field_empty("email") | credentials_submit.is_string_valid("email", 5)
    });
    let is_password_valid = use_memo(move || {
        credentials_submit.is_field_empty("password")
            | credentials_submit.is_string_valid("password", 6)
    });

    let sign_in_task = move |_| {
        spawn(async move {
            let app_state = APP_STATE.read();

            match sign_in(
                    credentials_submit.get_string("email"),
                    credentials_submit.get_string("password"),
                )
                .await
            { //app_state.auth.signal().set(auth_model)
                Ok(target) => nav.replace(target),
                Err(e) => {
                    //display modal
                    
                    nav.replace("/auth/login".to_string())
                }
            };
            is_busy.set(false);
        });
    };

    rsx! {
        div {
            class: "min-h-full relative flex flex-col justify-center overflow-hidden",
            div { class: "grow flex justify-center self-center z-10",
                div { class: "w-full p-6 m-auto bg-base-100 rounded-md shadow-md lg:max-w-lg",
                    div { class: "mb-2",
                        h3 { class: "font-semibold text-2xl text-base-700", "Sign In " }
                        p { class: "text-base-500", "Please sign in to your account." }
                    }
                    form { 
                        id: "credential-form",
                        class: "form-control",
                        oninput: move |event| credentials_submit.set(event.values()),
                        div {
                            label { class: "label",
                                span {
                                    class: "text-base label-text",
                                    "Email"
                                }
                            }
                            input {
                                placeholder: "mail@gmail.com",
                                name: "email",
                                r#type: "email",
                                id: "email",
                                class: "w-full input input-bordered input-primary",
                                class: if !is_login_valid() { "border-red-400 focus:border-red-400" },
                            }
                        }
                        label { class: "label",
                            span {
                                class: "text-base label-text",
                                "Password"
                            }
                        }
                        input {
                            r#type: "password",
                            name: "password",
                            id: "password",
                            placeholder: "Enter your password",
                            class: "w-full input input-bordered input-primary",
                            class: if !is_password_valid() { "border-red-400 focus:border-red-400" },
                        }
                        label { class: "label cursor-pointer justify-start",
                            input {
                                r#type: "checkbox",
                                name: "remember_me",
                                class: "checkbox checkbox-primary",
                                id: "remember_me"
                            }
                            label {
                                r#for: "remember_me",
                                class: "label-text ml-3",
                                "Remember me"
                            }
                        }
                        a {
                            href: "#",
                            class: "text-xs hover:underline hover:text-primary",
                            "Forgot your password?"
                        }
                        button {
                            onclick: sign_in_task,
                            r#type: "submit",
                            class: "btn btn-primary mt-5",
                            "Sign in",
                        }
                    }
                    div { class: "pt-5 text-center text-base-700 text-xs",
                        span {
                            "Copyright © 2024"
                            a {
                                target: "_blank",
                                rel: "",
                                class: "text-green hover:text-green-500",
                            }
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

#[server(Login)]
pub async fn sign_in(
    username: String,
    password: String,
) -> Result<String, ServerFnError> {
    println!("Server received: {} {}", username, password);

    let auth: crate::server::middlewares::auth::Session = extract().await.unwrap();

    //іnsert input validation here

    let stuff = crate::server::routers::auth::login_password(auth, username, password).await;
    Ok(stuff.headers()["Location"].to_str().unwrap_or("shit").to_string())
}


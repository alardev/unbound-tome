use dioxus::prelude::*;

#[component]
pub fn LoginButton() -> Element {
    let login = "Login";

    rsx!(
        div {
            tabindex: "0",
            id: "loginbutton",
            role: "button",
            class: "btn btn-ghost text-lg",
            aria_selected: "true",
            aria_controls: "tab-content",
            {
                login
            }
        }
    )
}

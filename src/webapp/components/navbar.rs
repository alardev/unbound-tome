use dioxus::prelude::*;

use crate::webapp::Route;

#[component]
pub fn NavBar() -> Element {

    let title = "Unbound Tome";
    let placeholder = "Search";
    let username = "Admin";
    let account = "My Account";

    rsx!(
        div { class: "navbar bg-base-100 grow-0",
              id: "navbar",
        div { class: "flex-1",
            Link {
                class: "btn btn-ghost text-xl",
                to: Route::Home {}, {title} } 
            
        }
        div { class: "flex-none gap-2",
            div { class: "form-control",
                input {
                    r#type: "text",
                    placeholder: placeholder,
                    class: "input input-bordered w-24 md:w-auto"
                }
            }
            div { class: "dropdown dropdown-end mx-auto",
                div {
                    tabindex: "0",
                    role: "button",
                    class: "btn btn-ghost",
                    div { class: "w-10 text-lg rounded-full mx-auto",
                        {username}
                    }
                }
                ul {
                    tabindex: "0",
                    class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow",
                    li {
                        a { class: "justify-between",
                            {"Profile"}
                            span { class: "badge", "New" }
                        }
                    }
                    li {
                        Link { to: Route::Account {}, {account} } 
                    }
                    li {
                        a {
                            "Logout"
                        }
                    }
                }
            }
        }
    }
    )    
}
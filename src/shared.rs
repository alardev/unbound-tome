#![allow(non_snake_case)]
#![forbid(unsafe_code)]

use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! (
        div {
            class: "flex h-screen flex-col",
            id: "content",
            {
                rsx! { Navbar {} }
            }
            div {
                id: "tab-content",
                role: "tabpanel",
                class: "bg-base-200 bg-cover flex flex-1",
                style: "background-image: url(https://i.etsystatic.com/18572829/r/il/bfb63e/4985394714/il_1140xN.4985394714_m67f.jpg);",
                div {
                    {}
                }
            }
            {
                rsx! { Footer {} }
            }
        }
    )
}

#[component]
pub fn Navbar() -> Element {

    let title = "Unbound Tome";
    let placeholder = "Search";

    rsx! {
        div {
            id: "navbar",
            class: "navbar bg-base-100", 
            div {
                class: "flex-1",
                    a {
                    class: "btn btn-ghost text-xl",
                    img {
                        class: "w-10 fill-white",
                        src: "/static/logo.svg",
                        {}
                    }
                    {title}
                }
            }
            div {
                class: "flex-none gap-2",
                div {
                    class: "form-control",
                    input {
                        r#type: "text",
                        placeholder: placeholder,
                        class: "input input-bordered w-24 md:w-auto",
                        {}
                    }
                }
                label {
                    class: "swap swap-rotate",
                    input {
                        r#type: "checkbox",
                        class: "theme-controller",
                        value: "light",
                        {}
                    }
                    svg {
                        class: "swap-on h-10 w-10 fill-current",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path {
                            d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z",
                            {}
                        }
                    }
                    svg {
                        class: "swap-off h-10 w-10 fill-current",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        path {
                            d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z",
                            {}
                        }
                    }
                }
                { 
                    rsx! { LogoutButton {} }
                }
            }
        }
    }
}

#[component]
pub fn LogoutButton() -> Element {
    let profile = "Profile";
    let new = "new";
    let settings = "Settings";
    let logout = "Logout";

    rsx!(
        div {
            id: "navbarlogin",
            class: "dropdown dropdown-end",
            div {
                tabindex: "0",
                role: "button",
                class: "btn btn-ghost text-lg",
                {
                    "admin"
                }
                ul {
                    class: "menu menu-sm dropdown-content
                            bg-base-100 rounded-box z-[1]
                            w-52 p-2 shadow",
                    li {
                        a {
                            class: "justify-between",
                            aria_selected: "true",
                            aria_controls: "tab-content",
                            {profile},
                            span {
                                class: "badge",
                                {
                                    new
                                }
                            }
                        }
                    }
                    li {
                        a {
                            {settings}
                        }
                    }
                    li {
                        id: "logoutbutton",
                        a {
                            role: "tab",
                            aria_selected: "true",
                            aria_controls: "tab-content",
                            {
                                logout
                            }
                        }
                    }
                }
            }
        }
    )
}

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


#[component]
pub fn Footer() -> Element {
    rsx!(
        footer {
            class: "footer bg-neutral text-neutral-content items-center p-4",
            aside {
                class: "grid-flow-col items-center",
                svg {
                    width: "36",
                    height: "36",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    class: "fill-current",
                    path {
                        d: "M22.672 15.226l-2.432.811.841 2.515c.33 1.019-.209 2.127-1.23 2.456-1.15.325-2.148-.321-2.463-1.226l-.84-2.518-5.013 1.677.84 2.517c.391 1.203-.434 2.542-1.831 2.542-.88 0-1.601-.564-1.86-1.314l-.842-2.516-2.431.809c-1.135.328-2.145-.317-2.463-1.229-.329-1.018.211-2.127 1.231-2.456l2.432-.809-1.621-4.823-2.432.808c-1.355.384-2.558-.59-2.558-1.839 0-.817.509-1.582 1.327-1.846l2.433-.809-.842-2.515c-.33-1.02.211-2.129 1.232-2.458 1.02-.329 2.13.209 2.461 1.229l.842 2.515 5.011-1.677-.839-2.517c-.403-1.238.484-2.553 1.843-2.553.819 0 1.585.509 1.85 1.326l.841 2.517 2.431-.81c1.02-.33 2.131.211 2.461 1.229.332 1.018-.21 2.126-1.23 2.456l-2.433.809 1.622 4.823 2.433-.809c1.242-.401 2.557.484 2.557 1.838 0 .819-.51 1.583-1.328 1.847m-8.992-6.428l-5.01 1.675 1.619 4.828 5.011-1.674-1.62-4.829z",
                        {}
                    }
                }
                p { "Copyright © - All right reserved" }
            }
        }
    )
}
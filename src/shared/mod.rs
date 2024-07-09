#![allow(non_snake_case)]
#![forbid(unsafe_code)]

pub mod routers;

use dioxus::prelude::*;
use routers::home::Home;
use routers::login::Login;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(NavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        Home {},

        // Wrap the next routes in a layout and a nest
        #[nest("/auth")]
        #[layout(Login)]
            #[route("/login")]
            Login {},
            // // At "/blog", we want to show a list of blog posts
            // #[route("/login")]
            // BlogList {},

        // We need to end the blog layout and nest
        // Note we don't need either - we could've just done `/blog/` and `/blog/:name` without nesting,
        // but it's a bit cleaner this way
        #[end_layout]
        #[end_nest]

    // And the regular page layout
    #[end_layout]


    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

pub fn App() -> Element {
    rsx! (
        Router::<Route> {}
    )
}

#[component]
pub fn NavBar() -> Element {

    let title = "Unbound Tome";
    let placeholder = "Search";
    let username = "Admin";

    rsx!(
        div { class: "navbar bg-base-100",
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
                        a { "Settings" }
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
    Outlet::<Route> {}
    { rsx!(Footer {}) }
    )    
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
            div {
                tabindex: "0",
                role: "button",
                class: "btn btn-ghost text-lg",
                {
                    "admin"
                }
                ul {
                    tabindex: "0",
                    class: "menu menu-sm 
                    dropdown-content
                    bg-base-100 rounded-box 
                    z-[1] mt-3 w-52 p-2 shadow",
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

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}


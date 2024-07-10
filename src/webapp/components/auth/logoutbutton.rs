use dioxus::prelude::*;

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
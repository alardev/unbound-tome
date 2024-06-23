use entity::appuser::Model;
use maud::{html, Markup};

pub fn render(
    user: std::option::Option<Model>,
) -> Markup {
    html! {
        main {
            header class="bg-background border-b-2 border-text" {
                nav class="flex justify-between items-center w-[92%] mx-auto" {
                    div class="flex" {
                        ion-icon name="book" class="text-5xl" href="/" {}
                        p class="text-3xl self-center" {"Unbound Tome"}
                    }
                    div class="nav-links duration-500 md:static 
                    absolute md:min-h-fit min-h-[60vh] left-0 top-[-100%] 
                    md:w-auto w-full flex items-center px-5" {
                        ul class="flex md:flex-row flex-col
                        md:items-center md:gap-[4vw] gap-8" {
                            li {
                                a class="hover:text-yellow-200" href="/campaigns" {
                                    "Campaigns"
                                }
                            }
                            li {
                                a class="hover:text-yellow-200" href="/characters" {
                                    "Characters"
                                }
                            }
                            li {
                                a class="hover:text-yellow-200" href="/resources" {
                                    "Resources"
                                }
                            }
                            li {
                                a class="hover:text-yellow-200" href="/about" {
                                    "About"
                                }
                            }
                            @if user.is_some() {
                                li id="logoutbutton" {
                                    a hx-get="/logout" role="tab"
                                    hx-target="#tab-content"
                                    aria-selected="true" 
                                    aria-controls="tab-content"
                                    class="bg-secondary px-5 py-2 
                                    rounded-full hover:bg-accent 
                                    cursor-pointer" {
                                        "Sign out"
                                    }
                                }

                                li id="accountbutton" {
                                    a hx-get="/account" role="tab"
                                    hx-target="#tab-content"
                                    aria-selected="true" 
                                    aria-controls="tab-content"
                                    class="bg-secondary px-5 py-2 
                                    rounded-full hover:bg-accent 
                                    cursor-pointer" {
                                        "My Account"
                                    }
                                }
                            } @else {
                                li id="loginbutton" {
                                    a hx-get="/login" role="tab"
                                    hx-target="#tab-content"
                                    aria-selected="true" 
                                    aria-controls="tab-content"
                                    class="bg-secondary px-5 py-2 
                                    rounded-full hover:bg-accent 
                                    cursor-pointer" {
                                        "Sign in"
                                    }
                                }
                            }
                        }
                    }
                    div class="flex items-center gap-6" {
                        ion-icon onclick="onToggleMenu(this)" name="menu" class="text-3xl cursor-pointer md:hidden" {}
                    }
                }
            }

            script {
                "const navLinks = document.querySelector('.nav-links')
                function onToggleMenu(e) {
                    e.name = e.name === 'menu' ? 'close' : 'menu'
                    navLinks.classList.toggle('top-[9%]')
                }"
            }
        }
    }
}
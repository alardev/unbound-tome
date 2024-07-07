use axum::Extension;
use maud::{html, Markup};

pub fn render(
    error_message: Option<String>,
    next: Option<String>
) -> Markup {
    html! {
        div class="w-full max-w-xs m-auto bg-background rounded p-5" {
            header {
                div class="flex" {
                    ion-icon
                    name="book"
                    class="text-5xl"
                    href="/"
                    {}
                }
                p class="text-3xl self-center" {
                    "Unbound Tome"
                }
            }

            form {
                label class="block mb-2" for="username" {
                    "Username"
                }
                input class="w-full p-2 mb-6
                text-accent border-b-2 
                border-accent outline-none 
                focus:bg-gray-300"
                type="text"
                name="username"
                id="username"
                placeholder="username"
                value="admin"
                {}
                label class="block mb-2" for="password" {
                    "Password"
                }
                input class="w-full p-2 mb-6
                text-accent border-b-2 
                border-accent outline-none 
                focus:bg-gray-300"
                type="password"
                name="password"
                id="password"
                placeholder="password"
                value="hunter42"
                {}
                button class="w-full 
                bg-secondary hover:bg-accent 
                cursor-pointer text-white 
                font-bold py-2 px-4 mb-6 
                rounded"
                hx-post="/login/password"
                hx-on--after-request=r##"console.log(this.detail.target)"##
                hx-target-error="#login-errors"
                hx-target="#tab-content" 
                {
                    "Sign In"
                }
                @if next.is_some() { 
                    input type="hidden" name="next" value=(next.unwrap()) {} 
                }
                div class="text-3xl self-center" id="login-errors" {
                    @if error_message.is_some() {
                        (error_message.unwrap())
                    }
                }
            }

            footer class="pb-4" {
                button class="hover:text-accent text-sm float-left"
                hx-get="/forgot"
                hx-push-url="true"
                hx-target="#tab-content"{
                    "Forgot Password?"
                }
                button class="hover:text-accent text-sm float-right"
                hx-get="/register"
                hx-push-url="true"
                hx-target="#tab-content" {
                    "Create Account"
                }
            }
        }
    }
}
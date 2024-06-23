use maud::{html, Markup};

pub mod shell;
pub mod navbar;
pub mod login;

pub fn page(body: Markup) -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                script src="https://cdn.tailwindcss.com" {}
                script src="https://unpkg.com/htmx.org@2.0.0" {}
                script src="https://unpkg.com/htmx-ext-response-targets@2.0.0/response-targets.js" {}

                script src="https://unpkg.com/ionicons@4.5.10-0/dist/ionicons.js" {}

                script {
                    "tailwind.config = {
                        theme: {
                          extend: {
                            colors: {
                            'text': 'oklch(92.64% 0.019 17.49)',
                            'background': 'oklch(12.53% 0.044 26.42)',
                            'primary': 'oklch(65.44% 0.139 21.60)',
                            'secondary': 'oklch(42.14% 0.157 27.34)',
                            'accent': 'oklch(49.14% 0.202 29.23)',
                            },
                          }
                        }
                    }"
                }

                link rel="icon mask-icon" href="/favicon.svg";
                link rel="manifest" href="/app.webmanifest";
                title { "Unbound Tome" }

                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta charset="utf-8";
            }

            body hx-ext="response-targets" class="bg-background text-text" {
                (body)
            }
        }
    }
}
use maud::{html, Markup};

pub fn render(
    hometitle: String,
    hometext: String,
    homebtn: String
) -> Markup {
    html! {
        div class="hero bg-base-200/0 flex-1"
        {
            div class="hero-content flex-col lg:flex-row-reverse" {
                div class="card lg:card-side lg:w-5/6 glass shadow-xl text-white" {
                    div class="card-body" {
                        h1 class="text-5xl card-title font-bold" {
                            (hometitle)
                        }
                        p class="text-2xl py-6" {
                            (hometext)
                        }
                        div class="card-actions justify-center" {
                            button class="btn btn-primary" {
                                (homebtn)
                            }
                        }
                    }
                    figure {
                        img src="https://i.pinimg.com/originals/d5/98/46/d59846b06d0dd2a415c07af101aaf055.png"
                        class="max-w-sm rounded-lg shadow-2xl"
                        {}
                    }
                }
            }
        }
    }
}
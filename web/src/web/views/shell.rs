use domains::users::model::Model;
use maud::{html, Markup};

use crate::web::views;

pub fn render(
    user: &std::option::Option<Model>,
    draw_content: Markup,
) -> Markup {
    html! {
        div 
        class="flex h-screen flex-col"
        id="content" {
            (views::navbar::render(&user))
            div
            id="tab-content"
            role="tabpanel" 
            class="bg-base-200 bg-cover flex flex-1"
            style="background-image: url(https://i.etsystatic.com/18572829/r/il/bfb63e/4985394714/il_1140xN.4985394714_m67f.jpg);"
            {
                (draw_content)
            }
            (views::footer::render())
        }
    }
}
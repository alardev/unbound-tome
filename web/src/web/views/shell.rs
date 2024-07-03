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
            div id="tab-content" 
            role="tabpanel" 
            class="bg-base-200 flex flex-1"
            {
                (draw_content)
            }
            (views::footer::render())
        }
    }
}
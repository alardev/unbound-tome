use domains::users::model::Model;
use maud::{html, Markup};

use crate::web::views;

pub fn render(
    user: &std::option::Option<Model>,
    draw_content: Markup,
) -> Markup {
    html! {
        div id="content" {
            (views::navbar::render(&user))
            div id="tab-content" 
            role="tabpanel" 
            class="container bg-slate-50/5 mx-auto 
            max-w-2xl mt-10 p-5 rounded-lg"
            {
                (draw_content)
            }
        }
    }
}
use crate::components::header::HeaderWidget;
use crate::components::{index_block::BlockWidget, index_hero::HeroWidget};
use perseus::{make_rx, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::prelude::{view, View};

// #[perseus::template_rx]
#[perseus::template_rx(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
            HeaderWidget()
        HeroWidget()
    //    BlockWidget()


                }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Share your link – Basic" }
    }
}

use perseus::{Html, RenderFnResultWithCause, Template};
// use sycamore::prelude::{view, Scope, View};
use sycamore::prelude::{view, Scope, SsrNode, View};

use crate::httpreq::model::LinkRx;

#[perseus::make_rx(LinkPageStateRx)]
// #[derive(serde::Serialize, serde::Deserialize)]
pub struct LinkPageState {
    pub greeting: String,
}
#[perseus::template(LinkPage)]
////#[perseus::template]
pub fn link_page<'a, G: Html>(cx: Scope<'a>, lk: LinkRx<'a>) -> View<G> {
    view! { cx,
        p { (lk.linkname.get()) }
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        //.build_state_fn(get_build_state)
        .template(link_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: LinkPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<LinkPageState> {
    Ok(LinkPageState {
        greeting: "Hello World!".to_string(),
    })
}

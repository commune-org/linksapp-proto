use perseus::{Html, RenderFnResultWithCause, Template};
// use sycamore::prelude::{view, Scope, View};
use sycamore::prelude::{view, Scope, SsrNode, View};

use crate::httpreq::model::Link;

#[perseus::make_rx(LinkPageStateRx)]
pub struct LinkPageState {
    // pub ls: Link,
    pub ls: String,
    pub path: String,
}
#[perseus::template_rx(LinkPage)]
////#[perseus::template]
pub fn link_page<'a, G: Html>(cx: Scope<'a>, lk: LinkPageStateRx<'a>) -> View<G> {
    view! { cx,
        p { (lk.ls.get()) }
        p { (lk.path.get()) }

        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("link")
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
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
    pth: String,
    _locale: String,
) -> RenderFnResultWithCause<LinkPageState> {
    let path = &pth;
    let lkx = format!("the path: link: {}", &path);
    Ok(LinkPageState {
        path: path.to_string(),
        ls: lkx,
    })
}

#[perseus::build_paths]
pub async fn get_build_paths() -> perseus::RenderFnResult<Vec<String>> {
    Ok(vec!["".to_string(), "lx".to_string()])
}

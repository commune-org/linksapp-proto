use sycamore::prelude::*;

use super::link_list::LinkListWidget;

#[component(MobileWidget<G>)]
pub fn MobileWidget<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);

    view! {cx,
                     div (class="d-flex flex-column bd-highlight mb-3 text-center") {
                         // h6{"@yoururl"}
                      div (class="p-2 bd-highlight") {a (class="btn btn-outline-primary", href="#") {"Link Layout1"}

                     }
           }
           LinkListWidget()
    }
}

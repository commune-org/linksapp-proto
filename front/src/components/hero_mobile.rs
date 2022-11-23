use sycamore::prelude::*;

use super::link_list::LinkListWidget;

#[component(MobileWidget<G>)]
pub fn MobileWidget<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);

    view! {cx,
                         // div (class="d-flex flex-column bd-highlight mb-3 text-center") {
                         //     // h6{"@yoururl"}
                         //  div (class="p-2 bd-highlight") {a (class="btn btn-outline-primary", href="#") {"Link Layout1"}

                         // }

                             h3 (class="bg-soft-primary text-center p-2") {
                                        "Flash Alx."
                                        div (class="container m-2") {
                                            i (class="bi bi-github") {}

                                            i (class="bi bi-linkedin") {}

                                            i (class="bi bi-discord") {}

                                        }

                                    }
                                    div (class="d-flex justify-content-center p-2") {
                                        div (class="card") {div (class="card-body") {"This This is John X, some text contents is some text within a card body and additional 3 linestext."}
    }

                                    }


               LinkListWidget()
        }
}

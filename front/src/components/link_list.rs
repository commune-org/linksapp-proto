use sycamore::prelude::*;

#[component(LinkListWidget<G>)]
pub fn LinkListWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
                                      li (class="list-group-item d-flex justify-content-between align-items-start") {
                                            div (class="ms-2 text-center me-auto") {
                                                p  {"Link Iitem 2 details"}
                                                span (class="badge bg-primary rounded-pill") {"GitHub"}
                                            }

                                        }

    }
}

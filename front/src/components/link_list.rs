use sycamore::prelude::*;

#[component(LinkListWidget<G>)]
pub fn LinkListWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

                                              div (class="container m-2") {
                                        ul (class="list-group") {li (class="list-group-item d-flex justify-content-between align-items-center") {
                                                "My latest update"
                                                span (class="badge bg-dark rounded-pill") {i (class="bi bi-github") {}
    }

                                            }

                                            li (class="list-group-item d-flex justify-content-between align-items-center") {
                                                "Come let's talk"
                                                span (class="badge bg-primary rounded-pill") {i (class="bi bi-discord") {}
    }

                                            }

                                            li (class="list-group-item d-flex justify-content-between align-items-center") {
                                                "An important event not to miss"
                                                span (class="badge bg-primary rounded-pill") {i (class="bi bi-calendar-date") {}
    }

                                            }

                                            li (class="list-group-item d-flex justify-content-between align-items-center") {
                                                "A new Software Release"
                                                span (class="badge bg-dark rounded-pill") {i (class="bi bi-github") {}
    }

                                            }

                                        }
    }

        }
}

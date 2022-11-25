use sycamore::prelude::*;

#[component(HeroProfile<G>)]
pub fn HeroProfile<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

            section (class="py-5 d-flex justify-content-center") {div (class="col-md-6") {
                    div (class="smartphone") {
                        div (class="device-header") {
                            div (class="proximity")

                            div (class="camera")

                            div (class="speaker")

                        }

                        div (class="content bg-primary bg-gradient bg-opacity-10") {
                            div (class="card text-center bg-transparent border-0 p-2") {
                                div (class="card-body p-1") {
                                    div (class="position-relative") {img (class="img-fluid avatar avatar-md-md rounded-circle shadow-lg w-25 h-25", src="assets/images/client/profile.png", alt="")
    }

                                }

                            }

                            h3 (class="bg-soft-primary text-center p-2") {
                                "Flash Alx."
                                div (class="container m-2") {
                                    i (class="bi bi-github")

                                    i (class="bi bi-linkedin")

                                    i (class="bi bi-discord")

                                }

                            }

                            div (class="d-flex justify-content-center p-2") {
                                div (class="card") {div (class="card-body") {"This This is John X, some text contents is some text within a card body and additional 3 linestext."}
    }

                            }

                            div (class="container m-2") {
                                ul (class="list-group") {li (class="list-group-item d-flex justify-content-between align-items-center") {
                                        "My latest update"
                                        span (class="badge bg-dark rounded-pill") {i (class="bi bi-github")
    }

                                    }

                                    li (class="list-group-item d-flex justify-content-between align-items-center") {
                                        "Come let's talk"
                                        span (class="badge bg-primary rounded-pill") {i (class="bi bi-discord")
    }

                                    }

                                    li (class="list-group-item d-flex justify-content-between align-items-center") {
                                        "An important event not to miss"
                                        span (class="badge bg-primary rounded-pill") {i (class="bi bi-calendar-date")
    }

                                    }

                                    li (class="list-group-item d-flex justify-content-between align-items-center") {
                                        "A new Software Release"
                                        span (class="badge bg-dark rounded-pill") {i (class="bi bi-github")
    }

                                    }

                                }
    }

                        }

                    }

                }

            }




                    }
}

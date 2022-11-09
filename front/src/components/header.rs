use sycamore::prelude::*;

#[component(HeaderWidget<G>)]
pub fn HeaderWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
                nav (class="navbar navbar-expand-lg navbar-light bg-light") {div (class="container px-4 px-lg-5") {
                a (class="navbar-brand", href="#!") {"LinksApp"}
                button (class="navbar-toggler", type="button", data-bs-toggle="collapse", data-bs-target="#navbarSupportedContent", aria-controls="navbarSupportedContent", aria-expanded="false", aria-label="Toggle navigation") {span (class="navbar-toggler-icon") {}
                }
            }
                }

    }
}

use sycamore::prelude::*;

#[component(HeroConfirm<G>)]
pub fn HeroConfirm<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

         div (class="d-flex justify-content-center") {div (class="shadow p-3 mb-5 bg-body rounded") {"Logged in Confirmed"} }
    }
}

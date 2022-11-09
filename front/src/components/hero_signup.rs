use sycamore::prelude::*;

#[component(HeroSignup<G>)]
pub fn HeroSignup<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    div (class="d-flex flex-column bd-highlight mb-3 text-center") {
        div (class="p-1 d-flex justify-content-center") {
            form  {
                div (class="mb-3 col-sm") {
                    input (class="form-control", id="exampleInputEmail1", type="email", aria-describedby="emailHelp", placeholder="username")
                }
                div (class="mb-3 col-sm") {
                    input (class="form-control", id="exampleInputPassword1", type="password", placeholder="password")
                }
                         button (class="btn btn-primary", type="submit") {"Submit"}
                                 }
                             }
                         }
         }
}

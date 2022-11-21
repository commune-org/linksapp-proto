use perseus::spawn_local_scoped;
use sycamore::prelude::*;

use super::index_hero::Block;

#[component(AddLinkWidget<G>)]
pub fn AddLinkWidget<G: Html>(cx: Scope) -> View<G> {
    // let update_node = move |x| {
    //     spawn_local_scoped(cx, async move { newb.set(x) });
    // };

    view! {cx,

           div (class="col-sm-7 p-3") {
               div (class="input-group mb-3") {
                   input (class="form-control", type="password",  placeholder="Password")
                   button( class="btn btn-outline-secondary", id="button-addon2", type="button") { "register" }
                   // button (on:click=move |_| update_node(Block::CreateLink), class="btn btn-outline-secondary", id="button-addon2", type="button") {"Add"}
                   //
            }

        }

    }
}

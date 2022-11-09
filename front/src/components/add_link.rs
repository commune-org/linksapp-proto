use perseus::spawn_local_scoped;
use sycamore::prelude::*;

use super::index_hero::Block;

#[component(AddLinkWidget<G>)]
pub fn AddLinkWidget<G: Html>(cx: Scope) -> View<G> {
    // let update_node = move |x| {
    //     spawn_local_scoped(cx, async move { newb.set(x) });
    // };

    view! {cx,

           div (class="col-sm-10 p-3") {
               div (class="input-group") {
                   span (class="input-group-text") {"TItle - Url"}
                   input (class="form-control", type="text", aria-label="Title", placeholder="Title") {}
                   input (class="form-control", type="text", aria-label="Url", placeholder="Url") {}
                   // button (on:click=move |_| update_node(Block::CreateLink), class="btn btn-outline-secondary", id="button-addon2", type="button") {"Add"}
            }

        }

    }
}

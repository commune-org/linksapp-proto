use perseus::{spawn_local_scoped, web_log};
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

use crate::httpreq::model::User;

use super::index_hero::Block;

#[component(AddLinkWidget<G>)]
pub fn AddLinkWidget<'a, G: Html>(cx: Scope<'a>, ul: &'a Signal<String>) -> View<G> {
    // pub fn AddLinkWidget<G: Html>(cx: Scope) -> View<G> {
    let usr = create_signal(cx, "".to_string());
    let pwd = create_signal(cx, "".to_string());

    let input_ref = create_node_ref(cx);

    let handle_usr = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        usr.set(target.value());
    };
    let handle_pwd = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        pwd.set(target.value());
    };

    let handle_submit = |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();

        if event.key() == "Enter" {
            let mut usr_t = usr.get().as_ref().clone();
            let mut pwd_t = usr.get().as_ref().clone();

            usr_t = usr_t.trim().to_string();
            pwd_t = pwd_t.trim().to_string();
            // web_log!("{:?}-{:?},{:?}", usr_t, pwd_t, ul.get());
            if !usr_t.is_empty() && pwd_t.is_empty() {
                // SAVE TO LOCALSTORAGE app_state.add_todo(task);
                usr.set("".to_string());
                pwd.set("".to_string());

                input_ref
                    .get::<DomNode>()
                    .unchecked_into::<HtmlInputElement>();

                //  .set_value("");
            }
        }
    };

    view! {cx,
           div{(ul.get())}
           div (class="col-sm-7 p-3") {
               div (class="input-group mb-3") {
                    input (ref=input_ref, on:keyup=handle_submit,  on:input=handle_usr, bind:value=usr, class="form-control", type="text", placeholder="email", )
                    input (ref=input_ref, on:keyup=handle_submit,  on:input=handle_pwd, bind:value=pwd, class="form-control", type="password", placeholder="password", )
                   // button( class="btn btn-outline-secondary", id="button-addon2", type="button") { "register" }
                   button(on:click=handle_submit, class="btn btn-outline-secondary", id="button-addon2", type="button") { "Create" }

                   // button (on:click=move |_| update_node(Block::CreateLink), class="btn btn-outline-secondary", id="button-addon2", type="button") {"Add"}
                   //
            }

        }

    }
}

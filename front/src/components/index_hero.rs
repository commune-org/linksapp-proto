use super::add_link::AddLinkWidget;
use super::hero_confirm::HeroConfirm;
use super::{hero_mobile::MobileWidget, hero_signup::HeroSignup};
use derive_more::Display;
use gloo_timers::future::TimeoutFuture;
use perseus::spawn_local_scoped;
use rand::Rng;
use std::error::Error as SysError;
use sycamore::prelude::*;
// use sycamore::suspense::{use_transition, Suspense};

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

#[derive(Debug, Clone, Display, Copy)]
pub enum Block {
    MobileLanding,
    CreateLink,
    LinkLists,
}

// #[cfg(target_arch = "wasm32")]
#[component(HeroWidget<G>)]
pub fn HeroWidget<G: Html>(cx: Scope) -> View<G> {
    let block = create_signal(cx, Block::MobileLanding);
    let rcblock = create_ref(cx, block);
    let user = create_signal(cx, "".to_string());

    let input_ref = create_node_ref(cx);

    let handle_input = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        user.set(target.value());
    };

    let handle_submit = |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();

        if event.key() == "Enter" {
            let mut task = user.get().as_ref().clone();
            task = task.trim().to_string();

            if !task.is_empty() {
                // SAVE TO LOCALSTORAGE app_state.add_todo(task);
                user.set("".to_string());
                input_ref
                    .get::<DomNode>()
                    .unchecked_into::<HtmlInputElement>();

                //  .set_value("");
            }
        }
    };

    // let update = move |x| block.set(x);
    // // let b = *block.get();
    let update_node = move |x| {
        spawn_local_scoped(cx, async move { block.set(x) });
    };

    let fetch_node = move |x| {
        #[cfg(target_arch = "wasm32")]
        spawn_local_scoped(cx, async move { block.set(x) });
    };

    view! {cx,

        section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
             div (class="row gx-4 gx-lg-5 align-items-center") {
                 div (class="col-md-6") {
                     h1 (class="display-5 fw-bolder p-3") {"Your Link"}
                     h1 (class="display-6 p-3") {"Share it accross"}
                     div (class="col-sm-7 p-3") {
                         div (class="input-group mb-3") {
                             input (ref=input_ref,  on:keyup=handle_submit,  on:input=handle_input, bind:value=user, class="form-control", type="text", placeholder="your link's url", aria-label="your link's url", aria-describedby="button-addon2") {}
                             button(on:click=move |_| update_node(Block::CreateLink), class="btn btn-outline-secondary", id="button-addon2", type="button") { "Create" }
                         }


                     }



                                                                           (
                                 match *block.get() {

                                     Block::CreateLink => AddLinkWidget(cx),
                                      _ => view!{cx, div{}}
                                     // Block::CreateLink => AddLinkWidget(cx),
                                     // Block::LinkLists => HeroConfirm(cx),
                                 }
                             )


                 }
                 div (class="col-md-6") {
                     div (class="smartphone") {
                         div (class="device-header") {
                             div (class="proximity") {}
                             div (class="camera") {}
                             div (class="speaker") {}
                         }
                         div (class="content bg-primary bg-gradient bg-opacity-10") {
                             div (class="card text-center bg-transparent border-0 p-3") {
                                 div (class="card-body p-0") {
                                     div (class="position-relative") {img (class="img-fluid avatar avatar-md-md rounded-circle shadow-lg w-25 h-25", src="assets/images/client/profile.png", alt="") {}
     }
                                 }
                             }
                             h3 (class="bg-soft-primary text-center") { (user.get())}


                             (
                                 match *block.get() {
                                     Block::MobileLanding => MobileWidget(cx),
                                     // Block::CreateLink => AddLinkWidget(cx),
                                     Block::LinkLists => HeroConfirm(cx),
                                      _ => view!{cx, div{}}
                                 }
                             )



                         }

                         div (class="device-footer") {
                             div (class="home-button") {}

                         }

                         }

                         }

                         }
             }


     }
    // script (src="assets/js/bootstrap.bundle.min.js")
     }
}

// button(on:click=move |_| update_node(Block::Two), class="btn btn-outline-secondary", id="button-addon2", type="button") { "Create" }
// (
//     match *block.get() {
//         Block::One => MobileWidget(cx),
//         Block::Two => HeroSignup(cx),
//         Block::Three => HeroConfirm(cx),
//     }
// )

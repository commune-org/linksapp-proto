use crate::httpreq::link::{add_link, add_status, link_status};
use crate::httpreq::model::User;

use super::add_link::AddLinkWidget;
use super::hero_confirm::HeroConfirm;
use super::hero_mobile::MobileWidget;

use derive_more::Display;
// use gloo_timers::future::TimeoutFuture;
use perseus::spawn_local_scoped;
//use rand::Rng;
// use std::error::Error as SysError;
use sycamore::prelude::*;
// use sycamore::suspense::{use_transition, Suspense};

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

#[derive(Debug, Clone)]
pub struct UserState {
    pub user: RcSignal<User>,
}

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
    let user = create_signal(cx, "".to_string());
    let usrlink = create_signal(cx, "".to_string());
    let input_ref = create_node_ref(cx);
    let req_status = create_signal(cx, 0);

    let handle_input = |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        usrlink.set(target.value());
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
    let update_node = move |x: String| {
        //#[cfg(target_arch = "wasm32")]
        spawn_local_scoped(cx, async move {
            // let fetch = fetch_user("mename").await.unwrap_or_default();
            // web_log!("{:?}", fetch.to_string());
            // req_status.set(fetch.to_string());
            // let resp = reqwasm::http::Request::get("http://localhost:8081/user/testname/")
            //     .send()
            //     .await
            //     .unwrap();
            //req_status.set(resp.status().to_string());
            // let status = 0
            // match status { 0 => Alert("server error"), 200 = > name exists already, 404 = > create_id }

            let stat = add_status(&x).await.unwrap_or_default();
            req_status.set(stat);
            // match *req_status.get() {
            //     // 404 => AddLinkWidget(cx, usrlink),
            //     404 => view! {cx, div{"not found"}},
            //     200 => view! {cx, div{"This account exists already"}},
            //     _ => view! {cx, div{""}}, //wicked bug! todo()
            //                               // Block::CreateLink => AddLinkWidget(cx),
            //                               // Block::LinkLists => HeroConfirm(cx),
            // }
        });
    };

    // // let update_node = move |x| {
    // //     spawn_local_scoped(cx, async move { block.set(x) });
    // // };

    view! {cx,

                section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
                     div (class="row gx-4 gx-lg-5 align-items-center") {
                         div (class="col-md-6") {
                             h1 (class="display-5 fw-bolder p-3") {"Your Link"}
                             h1 (class="display-6 p-3") {"Share it accross"}
                             div (class="col-sm-7 p-3") {
                                 div (class="input-group mb-3") {
                                     // input (ref=input_ref,  on:keyup=handle_submit,  on:input=handle_input, bind:value=usrlink, class="form-control", type="text", placeholder="your link's url")
                                     input (ref=input_ref,  on:input=handle_input, bind:value=usrlink, class="form-control", type="text", placeholder="your link's url")
                                     button(on:click=move |_| update_node(usrlink.get().to_string()), class="btn btn-outline-secondary", id="button-addon2", type="button") { "Create" }
                                     // button(on:click=move |_| update_node(Block::CreateLink), class="btn btn-outline-secondary", id="button-addon2", type="button") { "Create" }

                                 }



                             }
                                                                                   (

                                         match *req_status.get() {

                                             500 => view!{cx, div{"Accoutn exists already"}},
                                             // 200 => view!{cx, div{"Account Created"}},
                                             // 200 => AddLinkWidget(cx, usrlink),
                                              _ => view!{cx, div{""}}, //wicked bug! todo()
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



        (                             match *req_status.get() {
                                            // 200 => view!{cx,    h3{a(href = usrlink.get(), id = "about-link", class="bg-soft-primary text-center") { (usrlink.get()) } }  },
                                            200 => view!{cx,  h2(class="p-2 bd-highlight text-center") {a (class="btn btn-outline-primary btn-lg", href=usrlink.get()) {(usrlink.get())}}
                                                         p{} MobileWidget()
                                            },
                                              _ => view!{cx, div{""}}, //wicked bug! todo()
                                         }

        )
    div{""}
                                     (
                                         match *block.get() {
                                             // Block::MobileLanding => MobileWidget(cx),
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

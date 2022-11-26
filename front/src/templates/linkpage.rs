use perseus::{Html, RenderFnResultWithCause, Template};
// use sycamore::prelude::{view, Scope, View};
use sycamore::prelude::{view, Indexed, Keyed, Scope, SsrNode, View};

use crate::components::hero_profile::HeroProfile;
// use crate::httpreq::link::link_list;
use crate::httpreq::model::Link;

#[perseus::make_rx(LinkPageStateRx)]
pub struct LinkPageState {
    // pub ls: Link,
    pub ls: Vec<Link>,
    pub path: String,
}

#[perseus::template_rx(LinkPage)]
////#[perseus::template]
pub fn link_page<'a, G: Html>(cx: Scope<'a>, lk: LinkPageStateRx<'a>) -> View<G> {
    view! { cx,
         // p { (lk.ls.get()) }
          p { (lk.path.get()) }
     //         ul {
     //     Indexed(
     //         iterable=lk.ls,
     //         view=|cx, x| view! { cx,
     //             li { (x.linkname) }
     //         },
     //         //key=|x| *x.linkname,
     //     )
     // }
     //     a(href = "about", id = "about-link") { "About!" }
     // }

    HeroProfile()
     }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("link")
        .build_state_fn(get_build_state)
        .build_paths_fn(get_build_paths)
        // .request_state_fn(get_request_state)
        .incremental_generation()
        .template(link_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: LinkPageState) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    pth: String,
    _locale: String,
) -> RenderFnResultWithCause<LinkPageState> {
    let path = &pth;
    // let dyn_path = link_list().await.unwrap_or_default();
    // let vec_key: Vec<String> = dyn_path.iter().map(|p| p.linkname).collect();
    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
            // This just gets the IP address of the machine that built the app
            // let res = reqwest::get("https://api.ipify.org").await?.text().await?;
            // Ok::<String, reqwest::Error>(res)

            use hyper::{body::HttpBody, Client};
            use hyperlocal::{UnixClientExt, Uri};
            use std::error::Error;
            use tokio::io::{self, AsyncWriteExt as _};

            // let client = hyper::Client::new();

            // let uri = "http://httpbin.org/ip".parse().unwrap();
            // // Await the response...
            // let resp = client.get(uri).await?;
            // Ok::<String, hyper::Error>(resp.status().to_string())
            let pt = pth.as_str();

            let url = Uri::new("/tmp/linksapp-uds.socket", "/link/").into();

            let client = Client::unix();

            let mut response = client.get(url).await?;
            let mut bytes = Vec::default();
            while let Some(next) = response.data().await {
                let chunk = next?;
                bytes.extend(chunk);
            }

            let bres = String::from_utf8(bytes).unwrap();
            // let res = response.unwrap().status().to_string();
            let lnx: Vec<Link> = serde_json::from_str(&bres).unwrap();

            Ok::<Vec<Link>, hyper::Error>(lnx)

            // while let Some(next) = response {
            //     let chunk = next?;
            //     io::stdout().write_all(&chunk).await?;
            // }
        },
        true,
    )
    .await?;

    Ok(LinkPageState {
        path: path.clone(),
        ls: body,
    })
}

#[perseus::build_paths]
pub async fn get_build_paths() -> perseus::RenderFnResult<Vec<String>> {
    let body = perseus::utils::cache_fallible_res(
        "ipify",
        || async {
            use hyper::{body::HttpBody, Client};
            use hyperlocal::{UnixClientExt, Uri};
            use std::error::Error;
            use tokio::io::{self, AsyncWriteExt as _};

            let url = Uri::new("/tmp/linksapp-uds.socket", "/link/").into();

            let client = Client::unix();

            let mut response = client.get(url).await?;
            let mut bytes = Vec::default();
            while let Some(next) = response.data().await {
                let chunk = next?;
                bytes.extend(chunk);
            }

            let bres = String::from_utf8(bytes).unwrap();
            let lnx: Vec<Link> = serde_json::from_str(&bres).unwrap();

            let paths: Vec<String> = lnx.into_iter().map(|p| p.linkname).collect();

            Ok::<Vec<String>, hyper::Error>(paths)
        },
        true,
    )
    .await?;

    Ok(body)
}

// #[perseus::request_state]
// pub async fn get_request_state(
//     path: String,
//     _locale: String,
//     // Unlike in build state, in request state we get access to the information that the user sent
//     // with their HTTP request IN this example, we extract the browser's reporting of their IP
//     // address and display it to them
//     req: perseus::Request,
// ) -> RenderFnResultWithCause<LinkPageState> {
//     let body = perseus::utils::cache_fallible_res(
//         "ipify",
//         || async {
//             // This just gets the IP address of the machine that built the app
//             // let res = reqwest::get("https://api.ipify.org").await?.text().await?;
//             // Ok::<String, reqwest::Error>(res)

//             use hyper::{body::HttpBody, Client};
//             use hyperlocal::{UnixClientExt, Uri};
//             use std::error::Error;
//             use tokio::io::{self, AsyncWriteExt as _};

//             // let client = hyper::Client::new();

//             // let uri = "http://httpbin.org/ip".parse().unwrap();
//             // // Await the response...
//             // let resp = client.get(uri).await?;
//             // Ok::<String, hyper::Error>(resp.status().to_string())

//             let url = Uri::new("/tmp/linksapp-uds.socket", "/user/").into();

//             let client = Client::unix();

//             let mut response = client.get(url).await?;
//             let mut bytes = Vec::default();
//             while let Some(next) = response.data().await {
//                 let chunk = next?;
//                 bytes.extend(chunk);
//             }

//             let bres = String::from_utf8(bytes).unwrap();
//             // let res = response.unwrap().status().to_string();

//             Ok::<String, hyper::Error>(bres)

//             // while let Some(next) = response {
//             //     let chunk = next?;
//             //     io::stdout().write_all(&chunk).await?;
//             // }
//         },
//         true,
//     )
//     .await?;

//     Ok(LinkPageState {
//         path: path.clone(),
//         ls: format!("{:?}", body),
//     })
// }

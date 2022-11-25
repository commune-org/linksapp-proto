use gloo_net::http::Request;
// use serde_json::json;
use futures::future::join_all;

use super::model::Link;
use super::{BASE_API_URL, LINK_API};

// pub async fn get_story_preview(id: i64) -> Result<StoryItem> {
//     let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
//     Ok(Request::get(&url).send().await?.json().await?)
// }

pub async fn link_status(ln: &str) -> Result<u16, gloo_net::Error> {
    let url = format!("{}/{}/{}", BASE_API_URL, LINK_API, ln);
    let resp = Request::get(&url).send().await?;

    // let body = resp.json::<String>().await?;
    let req_status = resp.status();
    Ok(req_status)
}

pub async fn link_list() -> Result<String, gloo_net::Error> {
    let url = format!("{}/{}/", BASE_API_URL, LINK_API);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<String>().await?;
    //let req_status = resp.status();
    Ok(body)
}

pub async fn fetch_link(l: &str) -> Result<String, gloo_net::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, LINK_API, l);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<String>().await?;
    Ok(body)
}

pub async fn add_link(l: &str) -> Result<Link, gloo_net::Error> {
    let url = format!("{}/{}/{}", BASE_API_URL, LINK_API, l);
    let resp = Request::post(&url)
        .json(&Link {
            linkname: l.to_owned(),
        })
        .expect("fail to serialize json")
        .send()
        .await?;

    let body = resp.json::<Link>().await?;
    Ok(body)
}
pub async fn add_status(l: &str) -> Result<u16, gloo_net::Error> {
    let url = format!("{}/{}/", BASE_API_URL, LINK_API);
    let resp = Request::post(&url)
        .json(&Link {
            linkname: l.to_owned(),
        })
        .expect("fail to serialize json")
        .send()
        .await?;

    let req_status = resp.status();
    Ok(req_status)
}

async fn edit_link(l: &str) -> Result<Link, gloo_net::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, LINK_API, l);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<Link>().await?;
    Ok(body)
}

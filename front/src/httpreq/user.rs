use futures::future::join_all;
use reqwasm::http::Request;

use super::model::User;
use super::{BASE_API_URL, USER_API};

// pub async fn get_story_preview(id: i64) -> Result<StoryItem> {
//     let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
//     Ok(Request::get(&url).send().await?.json().await?)
// }

pub async fn user_status(link: &str) -> Result<u16, reqwasm::Error> {
    let url = format!("{}/{}/{}", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    // let body = resp.json::<String>().await?;
    let req_status = resp.status();
    Ok(req_status)
}

pub async fn fetch_user(link: &str) -> Result<String, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<String>().await?;
    Ok(body)
}

async fn add_user(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

async fn edit_user(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

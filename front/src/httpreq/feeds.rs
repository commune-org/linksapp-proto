async fn load_feeds(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

async fn add_feed(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

async fn edit_feed(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

async fn delete_feed(link: &str) -> Result<User, reqwasm::Error> {
    let url = format!("{}/{}/{}/", BASE_API_URL, USER_API, link);
    let resp = Request::get(&url).send().await?;

    let body = resp.json::<User>().await?;
    Ok(body)
}

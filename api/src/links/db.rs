use crate::errors::ServiceError;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::Link;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn link_add(client: &Client, selfobj: Link) -> Result<Link, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO public.users
   ( linkname )
    VALUES ($1) RETURNING linkname",
        )
        .await
        .unwrap();

    let res = client
        .query(
            &statement,
            &[
                // &selfobj.id,
                &selfobj.linkname,
            ],
        )
        .await?
        .last()
        .map(|row| Link::from_row_ref(row).unwrap());
    //        .collect::<Vec<CreateUsers>>()
    //        .pop();
    //.unwrap()?;
    // .ok_or(ServiceError::DuplicateValue(err))
    // .ok_or(io::Error::new(
    //
    //     io::ErrorKind::Other,
    //     "Error creating users tables",
    // ))
    // let maybe

    match res {
        Some(x) => Ok(x),
        None => Err(ServiceError::DuplicateValue("duplicate Value".to_string())), // Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
    // Ok(res)
}

/*
TODO populate fields
*/

pub async fn links_list(client: &Client) -> Result<Vec<Link>, io::Error> {
    let statement = client
        .prepare("select linkname from public.users order by id desc")
        .await
        .unwrap();

    let users_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Link::from_row_ref(row).unwrap())
        .collect::<Vec<Link>>();

    Ok(users_list)
}

pub async fn link_id(client: &Client, id_users: String) -> Result<Link, ServiceError> {
    let statement = client
        .prepare("select * from public.users where id = $1")
        .await
        .unwrap();

    let maybe_users = client
        .query_opt(&statement, &[&id_users])
        .await
        .expect("Error adding users ")
        .map(|row| Link::from_row_ref(&row).unwrap());

    match maybe_users {
        Some(users) => Ok(users),
        None => Err(ServiceError::NotFound("NotFound".to_string())),
    }
}

pub async fn single_link(client: &Client, id_users: String) -> Result<Link, ServiceError> {
    let statement = client
        .prepare("select * from public.users where linkname = $1")
        .await
        .unwrap();

    let maybe_users = client
        .query_opt(&statement, &[&id_users])
        .await
        .expect("Error finding users ")
        .map(|row| Link::from_row_ref(&row).unwrap());

    match maybe_users {
        Some(link) => Ok(link),
        None => Err(ServiceError::NotFound("NotFound".to_string())),
    }
}
//error retrieving column count: error deserializing column 0: cannot convert between the Rust type `core::option::Option<i8>` and the Postgres type `int8`',
pub async fn link_count(client: &Client, id_users: String) -> Result<i8, ServiceError> {
    let statement = client
        .prepare("select count(*) from public.users where linkname = $1")
        .await
        .unwrap();

    let maybe_count = client
        .query_one(&statement, &[&id_users])
        .await
        .expect("Error finding users ");

    let res = maybe_count.get("count");
    //println!("{:?}", &res);
    match res {
        Some(link) => Ok(link),
        None => Err(ServiceError::NotFound("NotFound".to_string())),
    }
}

//TODO take into account ID position

pub async fn link_update(client: &Client, id: i32, mdl: Link) -> Result<(), io::Error> {
    let statement = client
        .prepare("update public.users set ( linkname) = ($0,) where id = $1")
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&mdl.linkname])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn users_delete(client: &Client, users_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.users WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&users_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD

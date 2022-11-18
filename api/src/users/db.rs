use crate::errors::ServiceError;
use crate::users::{CreateUsers, User};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn users_add(client: &Client, selfobj: CreateUsers) -> Result<CreateUsers, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO public.users
   ( username, password, email)
    VALUES ($1, $2, $3) RETURNING id, username, password, email",
        )
        .await
        .unwrap();

    let res = client
        .query(
            &statement,
            &[
                // &selfobj.id,
                &selfobj.username,
                &selfobj.password,
                &selfobj.email,
            ],
        )
        .await?
        .last()
        .map(|row| CreateUsers::from_row_ref(row).unwrap());
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

pub async fn users_list(client: &Client) -> Result<Vec<User>, io::Error> {
    let statement = client
        .prepare("select * from public.users order by id desc")
        .await
        .unwrap();

    let users_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>();

    Ok(users_list)
}

pub async fn users_id(client: &Client, id_users: i32) -> Result<User, ServiceError> {
    let statement = client
        .prepare("select * from public.users where id = $1")
        .await
        .unwrap();

    let maybe_users = client
        .query_opt(&statement, &[&id_users])
        .await
        .expect("Error adding users ")
        .map(|row| User::from_row_ref(&row).unwrap());

    match maybe_users {
        Some(users) => Ok(users),
        None => Err(ServiceError::NotFound("NotFound".to_string())),
    }
}

//TODO take into account ID position

pub async fn users_update(client: &Client, id: i32, mdl: CreateUsers) -> Result<(), io::Error> {
    let statement = client.prepare("update public.users set (id, username, password, email) = ($0, $1, $2, $3) where id = $3").await.unwrap();

    let result = client
        .execute(&statement, &[&mdl.username, &mdl.password, &mdl.email, &id])
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

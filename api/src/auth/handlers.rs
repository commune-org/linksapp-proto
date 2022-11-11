use crate::auth::user::User;
// use crate::auth::{CallbackQuery, GetAuthUrlData, GoogleClient, Invitation, LoggedUser};
use crate::auth::{Invitation, LoggedUser, Token};
use crate::config::Config as Conf;
use crate::errors::ServiceError;
use actix_identity::Identity;
use actix_web::{
    http::StatusCode,
    web,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use chrono::Utc;
use futures::try_join;
use log::debug;
use serde::{Deserialize, Serialize};

use deadpool_postgres::{Client, Pool};
use uuid::Uuid;

pub type HttpResult = Result<HttpResponse, ServiceError>;
use crate::auth::auth::AuthRequest;
use crate::auth::AUTHORIZED_USERS;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateInvitation {
    pub email: String,
}

fn to_json<T>(js: T) -> HttpResult
where
    T: Serialize,
{
    Ok(HttpResponse::Ok().json(js))
}

fn form_http_response(body: String) -> HttpResult {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
}

pub async fn change_password_user(
    logged_user: LoggedUser,
    user_data: Json<UserData>,
    data: web::Data<Pool>,
) -> HttpResult {
    let client: Client = data.get().await.expect("Error connecting to the database");

    if let Some(mut user) = User::get_by_email(&logged_user.email, &client).await? {
        user.set_password(&user_data.password);
        user.update(&client).await?;
        form_http_response("password updated".to_string())
    } else {
        Err(ServiceError::BadRequest("Invalid User".into()))
    }
}

pub async fn register_email(
    invitation: Json<CreateInvitation>,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let config = Conf::from_env().unwrap();

    let domain = config.srv_cnf.host.clone();
    let email = invitation.into_inner().email;
    let invitation = Invitation::from_email(&email);
    invitation.inserts(db_pool).await?;
    invitation.send_confirmation_mail().await?;
    // invitation.send_invitation(&domain).await?;
    println!("{:#?}", &email);
    to_json(invitation)
}

pub async fn register_user(
    invitation_id: Path<String>,
    user_data: Json<UserData>,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let uuid = &invitation_id;
    let pool = db_pool.clone();

    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    if let invitation = Invitation::get_by_uuid(&uuid, db_pool).await? {
        if invitation.expires_at > Utc::now() {
            let user = User::from_details(&invitation.email, &user_data.password).await?;

            user.upsert(&client).await;

            invitation.delete(pool).await?;
            let user: LoggedUser = user.into();
            AUTHORIZED_USERS.store_auth(user.clone(), true)?;
            return to_json(user);
        } else {
            invitation.delete(pool).await?;
        }
    }
    Err(ServiceError::BadRequest("Invalid invitation".into()))
}

pub async fn login(
    auth_data: Json<AuthRequest>,
    id: Identity,
    db_pool: web::Data<Pool>,
) -> HttpResult {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    if let Some(user) = auth_data.authenticate(&client).await? {
        let user: LoggedUser = user.into();
        let token = Token::create_token(&user)?;
        id.remember(token.into());
        to_json(user)
    } else {
        Err(ServiceError::BadRequest("Invalid Login".into()))
    }
}

pub async fn logout(id: Identity) -> HttpResult {
    id.forget();
    if let Some(id) = id.identity() {
        form_http_response(format!("{} logged out", id))
    } else {
        form_http_response("".to_string())
    }
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResult {
    to_json(logged_user)
}

// TODO google_openId

// pub async fn auth_url(payload: Json<GetAuthUrlData>, client: Data<GoogleClient>) -> HttpResult {
//     let payload = payload.into_inner();
//     debug!("{:?}", payload.final_url);
//
//     let authorize_url = client.get_auth_url(payload).await?;
//     form_http_response(authorize_url.into_string())
// }

// pub async fn callback(
//     query: Query<CallbackQuery>,
//     data: web::Data<Pool>,
//     client: Data<GoogleClient>,
//     id: Identity,
// ) -> HttpResult {
//     let pool: Client = data.get().await.expect("Error connecting to the database");
//
//     if let Some((token, body)) = client.run_callback(&query, &pool).await? {
//         id.remember(token.into());
//         form_http_response(body.into())
//     } else {
//         Err(ServiceError::BadRequest("Callback Failed".into()))
//     }
// }

// TODO find number of non-confirmed invitations
// pub async fn status(data: Data<AppState>) -> HttpResult {
//     let ses = SesInstance::new(None);
//     let (number_users, number_invitations, (quota, stats)) = try_join!(
//         User::get_number_users(&data.pool),
//         Invitation::get_number_invitations(&data.pool),
//         ses.get_statistics(),
//     )?;
//     let body = format!(
//         "Users: {}<br>Invitations: {}<br>{:#?}<br>{:#?}<br>",
//         number_users, number_invitations, quota, stats,
//     );
//     form_http_response(body)
// }

use askama::Template;
use chrono::{DateTime, Duration, Utc};

use log::debug;
use postgres_query::FromSqlRow;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;
// use crate::templ;

use actix_session::Session;

use crate::config;
use crate::mail::SendMail;
use dotenv::dotenv;
use std::{env, io};

// use lettre::{SmtpClient, SmtpTransport, Transport};

// use crate::{app::CONFIG, errors::ServiceError as Error, pgpool::PgPool, ses_client::SesInstance};

use crate::auth::{is_signed_in, to_home};
use crate::errors::{ServiceError as Error, ServiceError};
use crate::templ::Register;
use actix_web::error::BlockingError;
use actix_web::web::Data;
use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, Pool};
// use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::error::Error as MailError;
// use lettre::smtp::error::Error as SmtpError;
// use lettre::smtp::response::Response;

// use lettre::smtp::ConnectionReuseParameters;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error as SmtpError;
// use lettre_email::error::Error as EmailError;
// use lettre_email::EmailBuilder;
use futures::TryFutureExt;
use std::error::Error as StdError;
use std::fmt;

// #[derive(Debug)]
// pub enum MailerError {
//     Smtp(SmtpError),
//     Mail(EmailError),
// }
//
// impl fmt::Display for MailerError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             MailerError::Smtp(ref err) => err.fmt(f),
//             MailerError::Mail(ref err) => err.fmt(f),
//         }
//     }
// }
//
// impl StdError for MailerError {
//     fn description(&self) -> &str {
//         match *self {
//             MailerError::Smtp(_) => "can not create email",
//             MailerError::Mail(_) => "can not build email",
//         }
//     }
// }
//
// impl From<SmtpError> for MailerError {
//     fn from(err: SmtpError) -> MailerError {
//         MailerError::Smtp(err)
//     }
// }
//
// impl From<EmailError> for MailerError {
//     fn from(err: EmailError) -> MailerError {
//         MailerError::Mail(err)
//     }
// }

#[derive(FromSqlRow, Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "invitation")]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: DateTime<Utc>,
}

impl Invitation {
    pub fn from_email(email: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at: Utc::now() + Duration::hours(24),
        }
    }

    pub async fn get_by_uuid(uuid: &str, pool: web::Data<Pool>) -> Result<Self, ServiceError> {
        let client: Client = pool.get().await.expect("Error connecting to the database");

        let uuid_path = Uuid::parse_str(uuid)?;
        let statement = client
            .prepare("SELECT * FROM invitations WHERE id = $id")
            .await
            .unwrap();

        let result = client
            .query_opt(&statement, &[&uuid_path])
            .await
            .expect("error getting invitations")
            .map(|rows| FromTokioPostgresRow::from_row(rows).unwrap());

        match result {
            Some(content) => Ok(content),
            None => Err(ServiceError::GenericError(String::from(
                "Could not get the ID",
            ))),
        }
    }

    // TODO modify client from web::data to postgres_pool

    pub async fn inserts(&self, pool: web::Data<Pool>) -> Result<(), ServiceError> {
        let client: Client = pool.get().await.expect("Error connecting to the database");

        let statement = client
            .prepare("INSERT INTO invitations (email, expires_at) VALUES ( $1, $2)")
            .await
            .unwrap();

        client
            .query(&statement, &[&self.email, &self.expires_at])
            .await
            .expect("Error Adding Invite");
        // print!("Invitation Successful");

        Ok(())
    }

    pub async fn delete(&self, pool: web::Data<Pool>) -> Result<(), ServiceError> {
        let client: Client = pool.get().await.expect("Error connecting to the database");
        let statement = client
            .prepare("DELETE FROM invitations WHERE id = $id")
            .await
            .unwrap();

        client.execute(&statement, &[&self.id]).await?;
        Ok(())
    }

    pub async fn send_confirmation_mail(&self) -> Result<Response, ServiceError> {
        dotenv().ok();
        let configs = config::Config::from_env().unwrap();

        let domain_url = format!("{}:{}", configs.srv_cnf.host, configs.srv_cnf.port);
        let expires = self
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string();

        let html_text = format!(
            "Please click on the link below to complete registration. <br/>
           <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
          This link expires on <strong>{expires}</strong>",
            domain = domain_url,
            id = self.id,
            expires = expires
        );
        let plain_text = format!(
            "Please visit the link below to complete registration:\n
          {domain}/register/{id}\n
          This link expires on {expires}.",
            domain = domain_url,
            id = self.id,
            expires = expires
        );

        let send_invite = SendMail {
            from_name: "Flash From".to_string(),
            from: "me@local.com".to_string(),
            reply_name: "The Same".to_string(),
            reply_to: "me@local.com".to_string(),
            to_name: "This User".to_string(),
            to: "user@local.com".to_string(),
            subject: "Invitation to join".to_string(),
            body: html_text,
        };

        let response = send_invite.send().await?;

        Ok(response)

        // let response = mailer.send(email.into()).map_err(ServiceError::Smtp)?;
        // mailer.close();
        // let email = EmailBuilder::new()
        //     .to(("user@example.org", "Firstname Lastname"))
        //     .from("user@example.com")
        //     .subject("Hi, Hello world")
        //     .text("Be happy!")
        //     .build()
        //     .unwrap();
        //
        // let creds = Credentials::new("".to_string(), "".to_string());
        //
        // // Open a remote connection to gmail
        // // let mailer = SmtpTransport::new().unwrap().credentials(creds).build();
        // // let mailer = SmtpTransport::new_encrypted_localhost()
        // //     .unwrap()
        // //     .transport();
        //
        // let mut mailer = SmtpClient::new_simple("localhost")
        //     .map_err(ServiceError::Smtp)?
        //     .credentials(creds)
        //     .smtp_utf8(true)
        //     .authentication_mechanism(Mechanism::Login)
        //     .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        //     .transport();

        // Send the email

        // let result = mailer.send(&email);

        // if result.is_ok() {
        //     println!("Email sent");
        // } else {
        //     println!("Could not send email: {:?}", result);
        //
        //     Err(Error::ProcessError(String::from(
        //         "Could not send confirmation email",
        //     )))
        // }
    }

    ///////////////////////////////////////

    pub async fn send_confirmation(
        &'static self,
        session: Session,
        //data: web::Json<RegisterData>,
        pool: web::Data<Pool>,
    ) -> Result<HttpResponse, ServiceError> {
        if is_signed_in(&session) {
            return Ok(HttpResponse::BadRequest().finish());
        }

        let confirmation = self.create_confirmation(pool).await;

        let result = web::block(move || confirmation).await;
        // let result = web::block(async { self.create_confirmation(pool).await });

        match result {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => match err {
                BlockingError::Error(auth_error) => Err(auth_error),
                BlockingError::Canceled => Err(ServiceError::GenericError(String::from(
                    "Could not complete the process",
                ))),
            },
        }
    }

    pub async fn show_confirmation_form(session: Session) -> Result<HttpResponse, ServiceError> {
        if is_signed_in(&session) {
            Ok(to_home())
        } else {
            let tpl = Register {
                sent: &false,
                error: &None,
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(tpl.render().unwrap()))
        }
    }

    pub async fn create_confirmation(
        &self,
        pool: web::Data<Pool>,
    ) -> Result<Response, ServiceError> {
        // let client = pool.get();
        self.mail_insert(pool).await;

        // self.inserts(pool);
        self.send_confirmation_mail().await
    }

    pub async fn mail_insert(&self, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
        let client: Client = pool.get().await.expect("Error connecting to the database");

        let statement = client
            .prepare(
                "INSERT INTO public.invitation
   (email)
    VALUES ($0) RETURNING id",
            )
            .await
            .unwrap();

        client
            .query(&statement, &[&self.email])
            .await
            .expect("Error creating content")
            .iter()
            .map(|row| Invitation::from_row_ref(row).unwrap())
            .collect::<Vec<Invitation>>()
            .pop()
            .ok_or(ServiceError::PostgressError(String::from(
                "Could not complete the process",
            )))
    }

    //
    // pub async fn send_invitation(&self, callback_url: &str) -> Result<(), Error> {
    //     let ses = SesInstance::new(None);
    //
    //     let sending_email = &CONFIG.sending_email_address;
    //0
    //     let email_body = format!(
    //         "Please click on the link below to complete registration. <br/>
    //          <a href=\"{url}?id={id}&email={email}\">
    //          {url}</a> <br>
    //          your Invitation expires on <strong>{exp}</strong>",
    //         url = callback_url,
    //         id = self.id,
    //         email = self.email,
    //         exp = self
    //             .expires_at
    //             .format("%I:%M %p %A, %-d %B, %C%y")
    //             .to_string(),
    //     );
    //
    //     ses.send_email(
    //         &sending_email,
    //         &self.email,
    //         "You have been invited to join Simple-Auth-Server Rust",
    //         &email_body,
    //     )
    //     .await
    //     .map(|_| debug!("Success"))
    //     .map_err(|e| Error::BadRequest(format!("Bad request {:?}", e)))
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::auth::Invitation;
//     use crate::{
//         app::{get_random_string, CONFIG},
//         errors::ServiceError as Error,
//         invitation::Invitation,
//         pgpool::PgPool,
//     };
//
//     #[tokio::test]
//     #[ignore]
//     async fn test_send_invitation() -> Result<(), Error> {
//         let new_invitation = Invitation::from_email("ddboline.im@gmail.com");
//
//         new_invitation.send_invitation("test_url").await?;
//         Ok(())
//     }
//
//     #[tokio::test]
//     #[ignore]
//     async fn test_create_delete_invitation() -> Result<(), Error> {
//         let pool = PgPool::new(&CONFIG.database_url);
//         let email = format!("{}@localhost", get_random_string(32));
//         let invitation = Invitation::from_email(&email);
//         let uuid = invitation.id.clone().to_string();
//         invitation.insert(&pool).await?;
//
//         let invitation = Invitation::get_by_uuid(&uuid, &pool).await?.unwrap();
//         println!("{:?}", invitation);
//
//         invitation.delete(&pool).await?;
//
//         assert!(Invitation::get_by_uuid(&uuid, &pool).await?.is_none());
//         Ok(())
//     }
// }

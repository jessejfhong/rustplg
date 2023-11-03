use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;
use crate::helpers::error_chain_fmt;
use crate::startup::ApplicationBaseUrl;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use aws_sdk_sesv2 as sesv2;
use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{PgPool, Postgres, Transaction};
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, pool, email_client, app_base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    app_base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, SubscribeError> {
    let new_subscriber = form.0.try_into().map_err(SubscribeError::ValidationError)?;

    //let transaction = pool.begin().await.context("Failed to acquire a Postgres connection from the pool.")?;

    let subscriber_id = insert_subscriber(&pool, &new_subscriber)
        .await
        .context("Failed to insert new subscriber in the database.")?;

    let subscription_token = generate_subscription_token();
    store_token(&pool, subscriber_id, &subscription_token)
        .await
        .context("Failed to store the confirmation token for a new subscriber.")?;

    //transaction.commit().await.context("Failed to commit SQL transaction to store a new subscriber.")?;

    send_confirmation_email(
        &email_client,
        &new_subscriber,
        &app_base_url.0,
        &subscription_token,
    )
    .await
    .context("Failed to send a confirmation email.")?;

    Ok(HttpResponse::Ok().finish())
}

// #[tracing::instrument(
//     name = "Store subscription token in the database.",
//     skip(transaction, subscription_token)
// )]
async fn store_token(
    transaction: &PgPool, // &Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"insert into subscription_tokens (subscription_token, subscriber_id)
           values ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(StoreTokenError)?;

    Ok(())
}

fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(new_subscriber, transaction)
// )]
pub async fn insert_subscriber(
    transaction: &PgPool, // &Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        insert into subscriptions (id, email, name, subscribed_at, status)
        values ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(transaction)
    .await?;

    Ok(id)
}

// #[tracing::instrument(
//     name = "Sending confirmation email to new subscriber.",
//     skip(client, new_subscriber, app_base_url)
// )]
pub async fn send_confirmation_email(
    client: &EmailClient,
    new_subscriber: &NewSubscriber,
    app_base_url: &str,
    subscription_token: &str,
) -> Result<SendEmailOutput, SendEmailError> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        app_base_url, subscription_token
    );

    let html_body = format!(
        "welcome to our newsletter!<br />\
        Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );

    let text_body = format!(
        "Welcone to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );

    client
        .send_email(&new_subscriber.email, "Welcome!", &html_body, &text_body)
        .await
        .map_err(SendEmailError)
}

pub struct StoreTokenError(sqlx::Error);

impl Debug for StoreTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Display for StoreTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while trying to store a subscription token."
        )
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

pub struct SendEmailError(aws_sdk_sesv2::Error);

impl Debug for SendEmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Display for SendEmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A error was encountered while sending email.")
    }
}

impl std::error::Error for SendEmailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),

    // #[error("Failed to acquire a Postgres connection from the pool.")]
    // PoolError(#[source] sqlx::Error),

    // #[error("Failed to insert new subscriber in the database.")]
    // InsertSubscriberError(#[source] sqlx::Error),

    // #[error("Failed to commit SQL transaction to store a new subscriber.")]
    // TransactionCommitError(#[source] sqlx::Error),

    // #[error("Failed to store the confirmation token for a new subscriber.")]
    // StoreTokenError(#[source] StoreTokenError),

    // #[error("Failed to send a confirmation email.")]
    // SendEmailError(#[source] SendEmailError),
    #[error(transparent)]
    UnexceptedError(#[from] anyhow::Error),
}

// impl From<SendEmailError> for SubscribeError {
//     fn from(e: SendEmailError) -> Self {
//         Self::SendEmailError(e)
//     }
// }

// impl From<StoreTokenError> for SubscribeError {
//     fn from(e: StoreTokenError) -> Self {
//         Self::StoreTokenError(e)
//     }
// }

impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            // SubscribeError::PoolError(_)
            // | SubscribeError::InsertSubscriberError(_)
            // | SubscribeError::TransactionCommitError(_)
            // | SubscribeError::StoreTokenError(_)
            // | SubscribeError::SendEmailError(_)
            SubscribeError::UnexceptedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Debug for SubscribeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

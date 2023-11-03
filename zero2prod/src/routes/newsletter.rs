use sha3::Digest;
use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::helpers::error_chain_fmt;
use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::http::{header, StatusCode};
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::fmt::{Debug, Display, Formatter};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PublishError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            PublishError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            PublishError::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);

                response
            }
        }
    }
}

impl Debug for PublishError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

struct Credentials {
    username: String,
    password: Secret<String>,
}

fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header is missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid utf8 string.")?;

    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization schema was not 'Basic'")?;

    let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode 'Basic' credentials.")?;

    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid utf8.")?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
        .to_string();

    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Credentials {
        username,
        password: Secret::new(password),
    })
}

async fn validate_credenticals(
    credentials: &Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, PublishError> {
    let password_hash = sha3::Sha3_256::digest(
        credentials.password.expose_secret().as_bytes()
    );
    let password_hash = format!("{:x}", password_hash);

    let user_id: Option<_> = sqlx::query!(
        r#"select user_id from users
           where username = $1 and password_hash = $2
        "#,
        credentials.username,
        password_hash
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to validate auth credentials.")
    .map_err(PublishError::UnexpectedError)?;

    user_id
        .map(|r| r.user_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid username or password."))
        .map_err(PublishError::AuthError)
}

#[tracing::instrument(
    name = "Publish email to confirmed subscriber.",
    skip(body, pool, email_client, request)
)]
pub async fn publish_newsletter(
    body: Json<BodyData>,
    pool: Data<PgPool>,
    email_client: Data<EmailClient>,
    request: HttpRequest,
) -> Result<HttpResponse, PublishError> {
    let credentials = basic_authentication(request.headers()).map_err(PublishError::AuthError)?;
    // log who is making the request
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    let user_id = validate_credenticals(&credentials, &pool).await?;
    // log who is making the request
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
    

    let subscribers = get_confirmed_subscriber(&pool)
        .await
        .context("Failed to get confirmed subscribers from database.")?;

    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send email to subscriber: {}.", subscriber.email)
                    })?;
            }

            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed subcriber. \
                    Their stored contact details are invalid",
                );
            }
        }
    }

    Ok(HttpResponse::Ok().finish())
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

//#[tracing::instrument(name = "Get confirmed subscriber", skip(pool))]
async fn get_confirmed_subscriber(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    // Demostrate how query_as is used, when return multiple fields

    // struct Row {
    //     email: String
    // }

    // let rows = sqlx::query_as!(
    //     Row,
    //     r#"select email from subscriptions where status = 'confirmed'"#
    // )
    // .fetch_all(pool)
    // .await?;

    // let confirmed_subscribers = rows
    //     .into_iter()
    //     .map(|r| match SubscriberEmail::parse(r.email) {
    //         Ok(email) => Ok(ConfirmedSubscriber { email }),
    //         Err(error) => Err(anyhow::anyhow!(error))
    //     })
    //     .collect();

    let confirmed_subscribers =
        sqlx::query!(r#"select email from subscriptions where status = 'confirmed'"#)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| match SubscriberEmail::parse(r.email) {
                Ok(email) => Ok(ConfirmedSubscriber { email }),
                Err(error) => Err(anyhow::anyhow!(error)),
            })
            .collect();

    Ok(confirmed_subscribers)
}

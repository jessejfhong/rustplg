use crate::helpers::error_chain_fmt;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use std::fmt::{Debug, Formatter};
use uuid::{uuid, Uuid};

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[derive(thiserror::Error)]
pub enum ConfirmEmailError {
    #[error("{0}")]
    TokenNotFoundError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for ConfirmEmailError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmEmailError::TokenNotFoundError(_) => StatusCode::UNAUTHORIZED,
            ConfirmEmailError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Debug for ConfirmEmailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument(name = "Confirm subscription.", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ConfirmEmailError> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Failed to get subscriber token from database.")?;

    match id {
        None => Err(ConfirmEmailError::TokenNotFoundError(format!(
            "Cannot find token: {}",
            parameters.subscription_token
        ))),
        Some(subscriber_id) => Ok(confirm_subscirber(&pool, subscriber_id)
            .await
            .map(|_| HttpResponse::Ok().finish())
            .context("Failed to update subscription.")?),
    }
}

async fn confirm_subscirber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"update subscriptions set status = 'confirmed' where id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"select subscriber_id from subscription_tokens where subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.subscriber_id))
}

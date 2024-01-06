use crate::jinja::JinjaAppState;
use crate::utils::e500;
use actix_web::{web, HttpResponse, HttpRequest};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;
use crate::authentication::UserId;
use minijinja::context;

pub async fn admin_dashboard(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    jinja_state: web::Data<JinjaAppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    Ok(jinja_state.render_template("admin_dashboard.html", &req, context! {username => username}))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(
    user_id: Uuid,
    pool: &PgPool
) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
        .fetch_one(pool)
        .await
        .context("Failed to perform a query to retrieve a username.")?;

    Ok(row.username)
}
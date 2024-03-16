use crate::http::ApiContext;
use crate::structs::RegisterRequest;
use crate::routes::web_routes::HtmlTemplate;
use crate::services::mail::{AuthToken, Email};

use askama::Template;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::debug_handler;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    routing::post,
    Extension, Form, Router,
};
use dotenv;
use sqlx::{types::Uuid, Error, PgPool};
use uuid::Uuid as uuid;

struct User {
    id: Uuid,
    email: String,
    num: String,
    prenum: String,
    agen_vehichel: bool,
    email_confirmed: bool,
}

#[derive(Default)]
struct AppState {
    pub email_service: AuthToken,
}

type SharedState = Arc<Mutex<AppState>>;

pub fn get_routes() -> Router {
    let email_service = SharedState::default();
    Router::new()
        .route("/register", post(register))
        .route("/user/:id", get(confirm_email))
        .with_state(Arc::clone(&email_service))
}

#[debug_handler]
async fn register(
    ctx: Extension<ApiContext>,
    State(state): State<SharedState>,
    Form(payload): Form<RegisterRequest>,
) -> Result<String, (StatusCode, String)> {
    if let Ok(user) = insert_participant(&payload, &ctx.db).await {
        let mut email = state.lock().await;
        let _ = email
            .email_service
            .send_email(
                &payload.email,
                format!(
                    "{}/api/user/{:?}",
                    dotenv::var("URI").expect("URI not set"),
                    user.id
                )
                .as_str(),
            )
            .await;
        return Ok("<div class=\"p-2 text-center\">cool che ti fas cun per serrar giu l'annunzia stos ti aunc confirmar tia email, en cuort survegnas ti ina email cun in link</div>".to_string());
    }
    eprintln!("Error adding user:");
    return Err((StatusCode::INTERNAL_SERVER_ERROR, "<div class=\"p-2 text-center\">Enzatgei ha buc funcziunau, emprova aunc inaga e sch'ei va buc scriva ina email al Maurus maurus.fry@gmail.com</div>".to_string()));
}

#[derive(Template)]
#[template(path = "pages/confirm_email.html")]
struct ConfirmEmailTemplate {
    success: bool,
}

#[debug_handler]
async fn confirm_email(ctx: Extension<ApiContext>, Path(id): Path<uuid>) -> impl IntoResponse {
    if let Ok(_) = sqlx::query_as!(
        User,
        "UPDATE participants SET email_confirmed = True where id = $1 RETURNING *",
        id
    )
    .fetch_one(&ctx.db)
    .await
    {
        let template = ConfirmEmailTemplate { success: true };
        return HtmlTemplate(template);
    }

    let template = ConfirmEmailTemplate { success: false };
    return HtmlTemplate(template);
}

async fn insert_participant(payload: &RegisterRequest, pool: &PgPool) -> Result<User, Error> {
    let agen_vehicel = payload.agen_vehichel.is_some();
    Ok(sqlx::query_as!(
        User,
        "INSERT INTO participants (num, prenum, email, agen_vehichel, email_confirmed)
        VALUES ($1, $2, $3, $4, False)
        RETURNING *",
        &payload.num,
        &payload.prenum,
        &payload.email,
        agen_vehicel
    )
    .fetch_one(pool)
    .await?)
}

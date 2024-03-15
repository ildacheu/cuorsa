use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

pub fn get_routes() -> Router {
    Router::new().route("/", get(get_register))
}

async fn get_register() -> impl IntoResponse {
    let template = RegisterTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/sign_up.html")]
struct RegisterTemplate;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", e),
            )
                .into_response(),
        }
    }
}

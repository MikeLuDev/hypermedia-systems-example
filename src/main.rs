use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse, Redirect},
    routing::get,
};

mod contact;
mod error;
mod templates;

use error::AppError;
use templates::IndexTemplate;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Result<impl IntoResponse, AppError> {
    let index_template = IndexTemplate {};
    let rendered = index_template.render()?;
    Ok(Html(rendered))
}

// If there is a search term found in the request, it will filter down to only contacts matching that term
// If not, it will simply list all contacts
async fn list_contacts() {}

async fn search_contact() {}

async fn create_contact() {}

async fn read_contact() {}

async fn update_contact() {}

async fn delete_contact() {}

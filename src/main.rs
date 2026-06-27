use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

mod contact;
mod error;
mod handlers;
mod state;
mod templates;

use contact::Contacts;
use tokio::sync::RwLock;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let contacts = Contacts::new();
    let app_state = AppState {
        contacts: Arc::new(RwLock::new(contacts)),
    };

    let app = Router::new()
        .route("/", get(handlers::contacts))
        .route("/contacts", get(handlers::contacts))
        .route("/contacts/new", get(handlers::new_contact))
        .route("/contacts/new", post(handlers::create_new_contact))
        .route("/contacts/{id}", get(handlers::get_contact))
        .route("/contacts/{id}/edit", get(handlers::edit_contact))
        .route("/contacts/{id}/edit", post(handlers::post_edit_contact))
        .route("/contacts/{id}/delete", post(handlers::delete_contact))
        .fallback(handlers::not_found)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

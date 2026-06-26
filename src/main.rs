use std::sync::{Arc};

use askama::Template;
use axum::{
    Form, Router,
    extract::{Query, State},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};

mod contact;
mod error;
mod templates;

use contact::Contacts;
use error::AppError;
use serde::Deserialize;
use templates::IndexTemplate;
use tokio::sync::RwLock;

use crate::{
    contact::{ContactErrors, NewContact},
    templates::NewContactTemplate,
};

#[derive(Clone)]
struct AppState {
    pub contacts: Arc<RwLock<Contacts>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let contacts = Contacts::new();
    let app_state = AppState {
        contacts: Arc::new(RwLock::new(contacts)),
    };

    let app = Router::new()
        .route("/", get(contacts_handler))
        .route("/contacts", get(contacts_handler))
        .route("/contacts/new", get(new_contact_page))
        .route("/contacts/new", post(create_new_contact))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ContactSearch {
    q: Option<String>,
}

async fn contacts_handler(
    State(state): State<AppState>,
    Query(params): Query<ContactSearch>,
) -> Result<impl IntoResponse, AppError> {
    let guard = state.contacts.read().await;
    let q = params.q.unwrap_or_default();

    let contacts = if q.is_empty() {
        guard.all()
    } else {
        guard.search(&q)
    };

    let template = IndexTemplate { contacts, q };
    Ok(Html(template.render()?))
}

async fn new_contact_page() -> Result<impl IntoResponse, AppError> {
    let template = NewContactTemplate {
        contact: NewContact::default(),
        errors: ContactErrors::default(),
    };
    Ok(Html(template.render()?))
}

async fn create_new_contact(
    State(state): State<AppState>,
    Form(form): Form<NewContact>,
) -> Result<Response, AppError> {
    let mut guard = state.contacts.write().await;

    let form_for_template = form.clone();

    let response = match guard.add(form) {
        Err(errors) => {
            let template = NewContactTemplate {
                contact: form_for_template,
                errors,
            };
            Html(template.render()?).into_response()
        }
        Ok(_) => Redirect::to("/contacts").into_response(),
    };

    Ok(response)
}

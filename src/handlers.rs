use askama::Template;
use axum::{
    Form,
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    contact::{Contact, ContactErrors, NewContact},
    error::AppError,
    state::AppState,
    templates::{
        EditContactTemplate, GetContactTemplate, IndexTemplate, NewContactTemplate,
        NotFoundTemplate,
    },
};

#[derive(Deserialize)]
pub struct ContactSearch {
    q: Option<String>,
}

pub async fn not_found() -> Result<impl IntoResponse, AppError> {
    Ok(Html(NotFoundTemplate {}.render()?))
}

pub async fn contacts(
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

pub async fn new_contact() -> Result<impl IntoResponse, AppError> {
    let template = NewContactTemplate {
        contact: NewContact::default(),
        errors: ContactErrors::default(),
    };
    Ok(Html(template.render()?))
}

pub async fn create_new_contact(
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

pub async fn get_contact(
    State(state): State<AppState>,
    Path(contact_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let guard = state.contacts.read().await;
    let contact = guard.get_by_id(contact_id);

    let template = match contact {
        None => NotFoundTemplate {}.render(),
        Some(contact) => GetContactTemplate { contact }.render(),
    };

    Ok(Html(template?))
}

pub async fn edit_contact(
    State(state): State<AppState>,
    Path(contact_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let guard = state.contacts.read().await;
    let contact = guard.get_by_id(contact_id);

    let template = match contact {
        None => NotFoundTemplate {}.render(),
        Some(contact) => EditContactTemplate {
            contact,
            errors: ContactErrors::default(),
        }
        .render(),
    };

    Ok(Html(template?))
}

pub async fn post_edit_contact(
    State(state): State<AppState>,
    Path(contact_id): Path<u64>,
    Form(form): Form<NewContact>,
) -> Result<Response, AppError> {
    let mut guard = state.contacts.write().await;
    let form_for_template = Contact {
        id: contact_id,
        first: form.first.clone(),
        last: form.last.clone(),
        phone: form.phone.clone(),
        email: form.email.clone(),
    };

    let response = match guard.edit(contact_id, form) {
        Err(errors) => {
            let template = EditContactTemplate {
                contact: form_for_template,
                errors,
            };
            Html(template.render()?).into_response()
        }
        Ok(_) => Redirect::to("/contacts").into_response(),
    };

    Ok(response)
}

pub async fn delete_contact(
    State(state): State<AppState>,
    Path(contact_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let mut guard = state.contacts.write().await;
    let _ = guard.delete(contact_id);

    Ok(Redirect::to("/contacts"))
}

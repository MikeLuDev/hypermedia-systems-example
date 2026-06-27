use std::sync::Arc;

use tokio::sync::RwLock;

use crate::contact::Contacts;

#[derive(Clone)]
pub struct AppState {
    pub contacts: Arc<RwLock<Contacts>>,
}

use askama::Template;
use std::sync::Arc;

use crate::models::{AuthenticatedUser, Repository};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub client_id: Arc<String>,
}

#[derive(Template)]
#[template(path = "logged_in_index.html")]
pub struct LoggedInIndexTemplate {
    pub user: AuthenticatedUser,
}

#[derive(Template)]
#[template(path = "repositories.html")]
pub struct RepositoriesTemplate {
    pub user: AuthenticatedUser,
    pub repositories: Vec<Repository>,
}

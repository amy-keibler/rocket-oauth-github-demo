use failure::{bail, Error};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};

use crate::models::{AuthenticatedUser, Repository};

pub fn retrieve_repositories(user: &AuthenticatedUser) -> Result<Vec<Repository>, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let get_request = client.get("https://api.github.com/user/repos");

    let response = user.send_authenticated_request(get_request);
    match response {
        Ok(mut response) => {
            let repositories: Vec<Repository> = response.json()?;
            Ok(repositories)
        }
        Err(e) => bail!(
            "Could not retrieve repositories for {}: {}",
            user.user_name,
            e
        ),
    }
}

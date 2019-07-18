use failure::{bail, Error};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

use crate::models::AuthenticatedUser;

pub struct OauthAppCredentials {
    pub client_id: Arc<String>,
    pub client_secret: Arc<String>,
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    scope: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubUser {
    name: String,
    email: Option<String>,
    avatar_url: Option<String>,
    gravatar_id: Option<String>,
}

pub fn authenticate<'a>(
    code: String,
    creds: &OauthAppCredentials,
) -> Result<AuthenticatedUser, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client
        .get("https://github.com/login/oauth/access_token")
        .query(&[
            ("client_id", creds.client_id.deref()),
            ("client_secret", creds.client_secret.deref()),
            ("code", &code),
        ])
        .send();

    if let Ok(mut res) = res {
        let auth: Result<AuthResponse, _> = res.json();
        if let Ok(auth) = auth {
            let res = client
                .get("https://api.github.com/user")
                .header("Authorization", String::from("token ") + &auth.access_token)
                .send();
            if let Ok(mut res) = res {
                let user: Result<GitHubUser, _> = res.json();

                if let Ok(user) = user {
                    return Ok(AuthenticatedUser::new(
                        user.email.unwrap_or_default(),
                        auth.access_token,
                        user.name,
                        user.avatar_url
                            .or(user
                                .gravatar_id
                                .map(|id| format!("https://www.gravatar.com/avatar/{}", id)))
                            .unwrap_or_default(),
                    ));
                } else {
                    bail!("Couldn't get user: {:?}", user);
                }
            } else {
                bail!("Got token, but failed to get email: {:?}", res);
            }
        } else {
            bail!("Got a response, but was not valid auth: {:?}", auth);
        }
    } else {
        bail!("Access flow failed");
    }
}

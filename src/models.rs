use failure::{format_err, Error};
use reqwest::RequestBuilder;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedUser {
    pub email: String,
    github_token: String,
    pub user_name: String,
    pub user_image: String,
}

impl AuthenticatedUser {
    pub fn new(email: String, github_token: String, user_name: String, user_image: String) -> Self {
        AuthenticatedUser {
            email,
            github_token,
            user_name,
            user_image,
        }
    }

    pub fn send_authenticated_request(
        &self,
        request: RequestBuilder,
    ) -> reqwest::Result<reqwest::Response> {
        request
            .header("Authorization", String::from("token ") + &self.github_token)
            .send()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let github_user: Result<AuthenticatedUser, Error> = request
            .cookies()
            .get_private("github_user")
            .map(|u| u.value().to_owned())
            .ok_or_else(|| format_err!("github_user cookie not present"))
            .and_then(|github_user| serde_json::from_str(&github_user).map_err(|e| e.into()));

        match github_user {
            Ok(github_user) => Outcome::Success(github_user),
            Err(_) => Outcome::Forward(()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
    pub html_url: String,
}

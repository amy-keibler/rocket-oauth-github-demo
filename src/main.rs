#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]

#[macro_use]
extern crate rocket;

extern crate askama;
extern crate serde;

use rocket::fairing::AdHoc;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::response::Redirect;
use rocket::State;

use rocket_contrib::serve::StaticFiles;

use log::error;

mod models;
use models::AuthenticatedUser;

mod auth;
use auth::{authenticate, OauthAppCredentials};

mod templates;
use templates::{IndexTemplate, LoggedInIndexTemplate};

#[get("/", rank = 1)]
fn logged_in_homepage(user: AuthenticatedUser) -> LoggedInIndexTemplate {
    LoggedInIndexTemplate { user }
}

#[get("/", rank = 2)]
fn homepage(creds: State<OauthAppCredentials>) -> IndexTemplate {
    IndexTemplate {
        client_id: creds.client_id.clone(),
    }
}

#[get("/auth_callback?<code>")]
fn auth_callback(
    code: String,
    mut cookies: Cookies,
    creds: State<OauthAppCredentials>,
) -> Redirect {
    let auth_user =
        authenticate(code, &creds).and_then(|u| serde_json::to_string(&u).map_err(|e| e.into()));

    match auth_user {
        Ok(auth_user) => cookies.add_private(cookie("github_user", auth_user)),
        Err(msg) => error!("Failed to authenticate: {}", msg),
    }

    Redirect::to("/")
}

#[get("/logout")]
fn log_out(_user: AuthenticatedUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("github_user"));

    Redirect::to("/")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![homepage, logged_in_homepage, auth_callback, log_out],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(AdHoc::on_attach("", |rocket| {
            let client_id = rocket
                .config()
                .get_str("client_id")
                .expect("Must configure OAuth Client ID")
                .to_string()
                .into();
            let client_secret = rocket
                .config()
                .get_str("client_secret")
                .expect("Must configure OAuth Client Secret")
                .to_string()
                .into();
            Ok(rocket.manage(OauthAppCredentials {
                client_id,
                client_secret,
            }))
        }))
        .launch();
}

fn cookie(key: &'static str, value: String) -> Cookie<'static> {
    Cookie::build(key, value).same_site(SameSite::Lax).finish()
}

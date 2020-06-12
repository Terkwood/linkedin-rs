//! Helper methods for several examples.
//!
//! The support files for each frontend include a client instance for several implemented
//! frontends. These are not part of the main example code as this library focusses purely on the
//! server side. This module contains code that can be shared between the different frontends.
//! Since we want to be able to run the the actix example while compiling with rocket features but
//! rocket includes macros in its crate root, the module include order is a bit strange.
//!
//! On supported systems (which have the `x-www-browser` command), there is a utility to open
//! a page in the browser.
#![allow(unused)]
use oxide_auth::endpoint::PreGrant;
use std::fmt;

pub use self::client::{Client, Config as ClientConfig, Error as ClientError};

/// Try to open the server url `http://localhost:8020` in the browser, or print a guiding statement
/// to the console if this is not possible.
pub fn open_in_browser() {
    use std::io::{Error, ErrorKind};
    use std::process::Command;

    let target_addres = "http://localhost:8020/";

    // As suggested by <https://stackoverflow.com/questions/3739327/launching-a-website-via-windows-commandline>
    let open_with = if cfg!(target_os = "linux") {
        // `xdg-open` chosen over `x-www-browser` due to problems with the latter (#25)
        Ok("xdg-open")
    } else if cfg!(target_os = "windows") {
        Ok("explorer")
    } else if cfg!(target_os = "macos") {
        Ok("open")
    } else {
        Err(Error::new(ErrorKind::Other, "Open not supported"))
    };

    open_with
        .and_then(|cmd| Command::new(cmd).arg(target_addres).status())
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, "Non zero status"))
            }
        })
        .unwrap_or_else(|_| println!("Please navigate to {}", target_addres));
}

pub fn consent_page_html(route: &str, grant: &PreGrant) -> String {
    macro_rules! template {
        () => {
"<html>'{0:}' (at {1:}) is requesting permission for '{2:}'
<form method=\"post\">
    <input type=\"submit\" value=\"Accept\" formaction=\"{4:}?response_type=code&client_id={3:}&allow=true\">
    <input type=\"submit\" value=\"Deny\" formaction=\"{4:}?response_type=code&client_id={3:}\">
</form>
</html>"
        };
    }

    format!(
        template!(),
        grant.client_id, grant.redirect_uri, grant.scope, grant.client_id, &route
    )
}

use self::actix_web::App;
use self::actix_web::*;
pub use self::generic::{consent_page_html, open_in_browser, Client, ClientConfig, ClientError};

pub fn dummy_client() -> App<Client> {
    let config = ClientConfig {
        client_id: "LocalClient".into(),
        protected_url: "http://localhost:8020/".into(),
        token_url: "http://localhost:8020/token".into(),
        refresh_url: "http://localhost:8020/refresh".into(),
        redirect_uri: "http://localhost:8021/endpoint".into(),
    };

    App::with_state(Client::new(config))
        .route("/endpoint", http::Method::GET, endpoint_impl)
        .route("/refresh", http::Method::POST, refresh)
        .route("/", http::Method::GET, get_with_token)
}

fn endpoint_impl(request: HttpRequest<Client>) -> HttpResponse {
    if let Some(cause) = request.query().get("error") {
        return HttpResponse::BadRequest()
            .body(format!("Error during owner authorization: {:?}", cause));
    }

    let code = match request.query().get("code") {
        None => return HttpResponse::BadRequest().body("Missing code"),
        Some(code) => code.clone(),
    };

    match request.state().authorize(&code) {
        Ok(()) => HttpResponse::Found().header("Location", "/").finish(),
        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
    }
}

fn refresh(request: HttpRequest<Client>) -> HttpResponse {
    match request.state().refresh() {
        Ok(()) => HttpResponse::Found().header("Location", "/").finish(),
        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
    }
}

fn get_with_token(request: HttpRequest<Client>) -> HttpResponse {
    let state = request.state();
    let protected_page = match state.retrieve_protected_page() {
        Ok(page) => page,
        Err(err) => return HttpResponse::InternalServerError().body(format!("{}", err)),
    };

    let display_page = format!(
        "<html><style>
            aside{{overflow: auto; word-break: keep-all; white-space: nowrap}}
            main{{text-align: center}}
            main>aside,main>article{{margin: auto; text-align: left; border: 1px solid black; width: 50%}}
        </style>
        <main>
        Used token <aside style>{}</aside> to access
        <a href=\"http://localhost:8020/\">http://localhost:8020/</a>.
        Its contents are:
        <article>{}</article>
        <form action=\"refresh\" method=\"post\"><button>Refresh token</button></form>
        </main></html>", state.as_html(), protected_page);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(display_page)
}

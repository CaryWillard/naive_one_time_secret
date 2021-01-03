extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;

mod crypto;
mod error;
mod models;
mod schema;
mod repository;

use actix_web::{
    App,
    HttpResponse,
    web,
    HttpServer,
};

use serde::Deserialize;

// "Naive" because it doesn't encrypt in the browser
// That might work with wasm

// MVP: Just an api, test with Postman or similar

// POST https://localhost:1337/secret/
// Body: secret=<secret-text-unencrypted> // form data
// Returns: a code, which is a hash of the secret

// GET: https://localhost:1337/secret/?code=<hash-of-secret>
// Returns: <secret-text-unencrypted>

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    sodiumoxide::init().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/secret")
                    .route(web::post().to(create_secret))
                    .route(web::get().to(get_secret))
                    .app_data(
                        web::JsonConfig::default()
                            .content_type(|mime| {
                                mime.type_() == "text" && mime.subtype() == "plain"
                            })
                            .error_handler(|err, _req| {
                                actix_web::error::InternalError::from_response(
                                    err, HttpResponse::Conflict().finish()
                                ).into()
                            })
                    )
            )
    })
    .bind("127.0.0.1:1337")?
    .run()
    .await
}

#[derive(Deserialize)]
struct CreateSecretBody {
    secret: String,
}

async fn create_secret(secret: web::Json<CreateSecretBody>) -> String {
    format!("{}", secret.secret)
}

#[derive(Deserialize)]
struct GetSecretQuery {
    code: String
}

async fn get_secret(query: web::Query<GetSecretQuery>) -> String {
    format!("{}", query.code)
}

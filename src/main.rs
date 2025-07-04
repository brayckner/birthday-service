mod controller;
mod service;

use crate::controller::hello_controller::hello_world;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(
                    Cors::default()
                        .allowed_origin("http://localhost:5173")
                        .allowed_methods(vec!["GET", "OPTIONS"])
                        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                        .allowed_header(header::CONTENT_TYPE)
                        .max_age(3600)
                )
                .service(hello_world)
        );
    };

    Ok(config.into())
}

use std::sync::Arc;
use actix_web::{App, HttpServer, middleware};
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::web::{Data, get};
use crate::api::routers;
use crate::di;
use crate::di::provider::Container;

pub fn create_server() -> Result<Server, std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    log::info!("Starting HTTP server: go to http://localhost:8080");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(di::provider::inject()))
            .wrap(middleware::Logger::default())
            .route("/", get().to(|| async { "Hello Rust!" }))
            .configure(routers::initialize)
    })
        .bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}
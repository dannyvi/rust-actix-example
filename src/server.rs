//! Spin up a HTTPServer

// use crate::auth::get_identity_service;
use crate::cache::add_cache;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use crate::state::new_state;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer, http::header};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>
    let data = new_state::<String>();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .configure(add_cache)
            .wrap(
                Cors::default()
                .allowed_origin(&format!("http://{}", &CONFIG.server))
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600),
                // Cors::new().supports_credentials().finish()
            )
            .wrap(Logger::default())
            // .wrap(get_identity_service())
            .configure(add_pool)
            .app_data(data.clone())
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}

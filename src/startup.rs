use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;

#[macro_export]
macro_rules! build_app {
    () => {
        App::new()
            .route(
                "/health_check",
                web::get().to($crate::routes::health_check::health_check),
            )
            .route(
                "/subscribe",
                web::post().to($crate::routes::subscribe::subscribe),
            )
    };
}

pub async fn run_app(host: String, port: u16, connection_pool: MySqlPool) -> std::io::Result<()> {
    // wrap connection in a data so that it can be accessed in the request handlers
    let connection_pool = web::Data::new(connection_pool);

    HttpServer::new(move || build_app!().app_data(connection_pool.clone()))
        .bind((host, port))?
        .run()
        .await
}

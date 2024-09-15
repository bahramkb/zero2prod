use actix_web::{web, App, HttpServer};

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

pub async fn run_app(host: String, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| build_app!())
        .bind((host, port))?
        .run()
        .await
}

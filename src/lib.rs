use actix_web::{web, App, HttpServer};

pub mod routes;

#[macro_export]
macro_rules! build_app {
    () => {
        App::new().route(
            "/health_check",
            web::get().to($crate::routes::health_check::health_check),
        )
    };
}

pub async fn run_app() -> std::io::Result<()> {
    HttpServer::new(|| build_app!())
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

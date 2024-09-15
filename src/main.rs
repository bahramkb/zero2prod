use sqlx::mysql::MySqlPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = MySqlPoolOptions::new()
        .min_connections(3)
        .max_connections(10)
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to MySQL.");

    println!(
        "Starting server at http://{}:{}",
        configuration.app.host, configuration.app.port
    );
    run_app(
        configuration.app.host,
        configuration.app.port,
        connection_pool,
    )
    .await
}

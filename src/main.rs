use zero2prod::configuration::get_configuration;
use zero2prod::startup::run_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    println!(
        "Starting server at http://{}:{}",
        configuration.app.host, configuration.app.port
    );
    run_app(configuration.app.host, configuration.app.port).await
}

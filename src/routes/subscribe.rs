use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscriptionFormData>,
    connection_pool: web::Data<MySqlPool>,
) -> impl Responder {
    println!("{:#?}", form);

    match sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES (?, ?, ?, ?)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(connection_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use sqlx::mysql::MySqlPoolOptions;

    use super::*;
    use crate::build_app;
    use crate::configuration::get_configuration;

    impl SubscriptionFormData {
        fn new(name: String, email: String) -> Self {
            Self { name, email }
        }
    }

    #[actix_web::test]
    async fn subscribe_returns_200_for_valid_form_data() {
        let test_data =
            SubscriptionFormData::new(String::from("bahram"), String::from("test@gmail.com"));

        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_string = configuration.database.connection_string();

        let connection_pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&connection_string)
            .await
            .expect("Failed to connect to MySQL.");
        let connection_pool = web::Data::new(connection_pool);
        let app = test::init_service(build_app!().app_data(connection_pool.clone())).await;

        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_form(&test_data)
            .to_request();
        let response = test::call_service(&app, req).await;

        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        assert_eq!(
            body,
            format!(
                "Successfully subscribed name: {}, email: {}",
                test_data.name, test_data.email
            )
        );

        let saved = sqlx::query!(
            "SELECT * FROM subscriptions WHERE email = ?",
            test_data.email
        )
        .fetch_one(connection_pool.get_ref())
        .await
        .expect("Failed to fetch saved subscription.");

        assert_eq!(saved.email, test_data.email);
        assert_eq!(saved.name, test_data.name);
    }
    #[actix_web::test]
    async fn subscribe_returns_400_when_data_is_missing() {
        let app = test::init_service(build_app!()).await;

        let test_cases = vec![
            ("name=bah%20ram", "missing the email"),
            ("email=test%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            let req = test::TestRequest::post()
                .uri("/subscribe")
                .append_header(("Content-Type", "application/x-www-form-urlencoded"))
                .set_payload(invalid_body)
                .to_request();
            let response = test::call_service(&app, req).await;

            assert!(
                response.status().is_client_error(),
                "Subscribe API did not fail with 400 Bad Request when payload was {}.",
                error_message
            );
        }
    }
}

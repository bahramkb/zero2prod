use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionFormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscriptionFormData>) -> impl Responder {
    println!("{:#?}", form);
    HttpResponse::Ok().body(format!(
        "Successfully subscribed name: {}, email: {}",
        form.name, form.email
    ))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::build_app;
    use crate::routes::subscribe::SubscriptionFormData;

    impl SubscriptionFormData {
        fn new(name: String, email: String) -> Self {
            Self { name, email }
        }
    }

    #[actix_web::test]
    async fn subscribe_returns_200_for_valid_form_data() {
        let test_data =
            SubscriptionFormData::new(String::from("bahram"), String::from("test@gmail.com"));

        let app = test::init_service(build_app!()).await;
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

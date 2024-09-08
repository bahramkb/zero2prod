use actix_web::{HttpResponse, Responder};
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::build_app;
    #[actix_web::test]
    async fn dummy_test() {
        let app = test::init_service(build_app!()).await;
        let req = test::TestRequest::get().uri("/health_check").to_request();
        let response = test::call_service(&app, req).await;

        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        assert_eq!(body, "OK");
    }
}

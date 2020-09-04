// src/test.rs
use crate::db;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[cfg(test)]
pub fn init() {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        dotenv().ok();
        db::establish_connection();
        *initiated = true;
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::models::demo::Demos;
    use crate::routes::*;
    use actix_web::{
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_rt::test]
    async fn test_user() {
        super::init();

        let request_body = json!({
            "name": "good name",
            "demo_text": "This is a great rust demo!",
            "favorite_number": 8,
        });

        let mut app = test::init_service(App::new().configure(build_routes)).await;

        let req = TestRequest::post()
            .uri("/demo")
            .set_json(&request_body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success(), "Failed to create demo");

        let req = TestRequest::post()
            .uri("/demo")
            .set_json(&request_body)
            .to_request();

        let write_demo: Demos = test::read_response_json(&mut app, req).await;
        assert_eq!(write_demo.id.is_positive(), true);

        let req = TestRequest::get()
            .uri(&format!("/demo/{}", write_demo.id))
            .set_json(&request_body)
            .to_request();

        let read_demo: Demos = test::read_response_json(&mut app, req).await;
        assert_eq!(read_demo, write_demo);
    }
}

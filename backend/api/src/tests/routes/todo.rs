// use actix_http::StatusCode;
// use actix_web::App;
// use fake::{
//   faker::internet::en::{FreeEmail, Password, SafeEmail, Username},
//   Fake,
// };
// use shared::models::{CreateTodo, CreateUser};

// use crate::routes::todo::service;

// #[actix_rt::test]
// async fn post_todo_works() {
//   let app = App::new().configure(service);
//   let mut app = actix_web::test::init_service(app).await;

//   let new_user = CreateUser {
//     name: Username().fake(),
//     email: FreeEmail().fake(),
//     password: Password(12..18).fake(),
//   };

//   // using unwrap: trust me it'S safe :)
//   let req = reqwest::post()
//   let req = actix_web::test::TestRequest::post()
//     .uri("/api/v1/users")
//     .set_json(serde_json::to_string(&new_user).unwrap())
//     .to_request();

//   let res = actix_web::test::call_service(&mut app, req).await;

//   let new_todo = CreateTodo {
//     title: "test".to_string(),
//     description: "test".to_string(),
//     ..Default::default()
//   };

//   assert!(res.status().is_success());
//   assert_eq!(res.status(), StatusCode::OK);
//   let data = res.headers().get("version").and_then(|h| h.to_str().ok());
//   assert_eq!(data, Some(API_VERSION));
// }

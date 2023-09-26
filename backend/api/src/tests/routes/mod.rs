// use fake::{
//   faker::internet::en::{FreeEmail, Password, SafeEmail, Username},
//   Fake,
// };
// use shared::models::{CreateUser, User};

// mod health;
// mod todo;

// pub const API_URL: &str = "http://127.0.0.1:8000/api/v1";
// pub const SERVER_URL: &str = "http://127.0.0.1:8000";
// pub struct TestWebInstance {
//   pub user: User,
//   pub reqw: reqwest::Client,
// }

// pub async fn create_user_mock() -> CreateUser {
//   CreateUser {
//     name: Username().fake(),
//     email: SafeEmail().fake(),
//     password: Password(12..18).fake(),
//   }
// }

// impl TestWebInstance {
//   pub async fn new() -> Self {
//     let reqw = reqwest::Client::builder()
//       .redirect(reqwest::redirect::Policy::none())
//       .cookie_store(true)
//       .build()
//       .unwrap();

//     let create_user = create_user_mock().await;

//     let new_user_res = reqw
//       .post(&format!("{}/users", API_URL))
//       .json::<CreateUser>(&create_user)
//       .send()
//       .await
//       .unwrap();

//     let user = new_user_res.json::<User>().await.unwrap();

//     Self { user, reqw }
//   }
// }

// pub async fn spawn_app(pool: sqlx::postgres::PgPool) -> TestWebInstance {
//   dotenv::from_filename(".env.test").ok();
//   let app = App::new().configure(service);
//   let mut app = actix_web::test::init_service(app).await;

//   let _ = tokio::spawn(application.run_until_stopped());

//   let test_web_inst = TestWebInstance::new().await;

//   test_app.test_user.store(&pool).await;

//   test_app
// }

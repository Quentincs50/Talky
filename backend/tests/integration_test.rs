use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;
use serde_json::{json, Value};
use tokio;
use sqlx::postgres::PgPoolOptions;

// On importe TOUT depuis ta librairie backend
use backend::routes::create_router; 

#[tokio::test]
async fn full_flow_integration_test() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // On force le type pour éviter l'erreur d'inférence E0282
    let app: Router = create_router().await;

    let unique_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let test_email = format!("test_{}@example.com", unique_id);

    // 1. SIGNUP (Teste user_services::create_user)
    let signup_payload = json!({
        "username": "IvanaTest",
        "email": test_email,
        "password": "Password123"
    });

    let response = app.clone().oneshot(
        Request::builder().method("POST").uri("/auth/signup")
            .header("content-type", "application/json")
            .body(Body::from(signup_payload.to_string())).unwrap(),
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. LOGIN (Teste user_services::authenticate_user)
    let login_payload = json!({"email": test_email, "password": "Password123"});
    let response = app.clone().oneshot(
        Request::builder().method("POST").uri("/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(login_payload.to_string())).unwrap(),
    ).await.unwrap();
    
    let body_bytes = axum::body::to_bytes(response.into_body()).await.unwrap();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = body_json["token"].as_str().expect("Token missing").to_string();

    // 3. CREATE SERVER (Teste server_services)
    let response = app.clone().oneshot(
        Request::builder().method("POST").uri("/servers")
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::from(json!({"name": "Test Server"}).to_string())).unwrap(),
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
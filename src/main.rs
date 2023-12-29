use axum::{routing::get, Router};
use diesel::RunQueryDsl;
use my_axum_app::*;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = database::establish_connection();
    let mut connection = pool.get().expect("Failed to get connection from pool");

    match diesel::sql_query("SELECT 1").execute(&mut connection) {
        Ok(_) => println!("Database connection successful!"),
        Err(err) => eprintln!("Error connecting to the database: {:?}", err),
    }

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "hello, rust" }))
        .nest("/api", routes::api_routes())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("{:?}", listener);
    axum::serve(listener, app).await.unwrap();
}

use axum::{routing::get, Router};
use diesel::RunQueryDsl;
use my_axum_app::*;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let connection = &mut establish_connection();

    match diesel::sql_query("SELECT 1").execute(connection) {
        Ok(_) => println!("Database connection successful!"),
        Err(err) => eprintln!("Error connecting to the database: {:?}", err),
    }

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "hello, rust" }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("{:?}", listener);
    axum::serve(listener, app).await.unwrap();
}

use axum::{routing::get, Router};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // 데이터베이스 연결 풀 설정
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("111");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    println!("2222");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    println!("333");

    // Get a connection from the pool
    let mut connection = pool.get().expect("Failed to get connection from pool");

    // Use the connection to execute a simple query
    match diesel::sql_query("SELECT 1").execute(&mut connection) {
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

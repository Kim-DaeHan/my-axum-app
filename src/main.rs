use axum::{routing::get, Router};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use std::net::SocketAddr;
fn main() {
    dotenv::dotenv().ok();

    // 데이터베이스 연결 풀 설정
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // 데이터베이스 연결 풀 설정
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // let app = Router::new().nest("/api", Router::route(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

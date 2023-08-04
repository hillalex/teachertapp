use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::Path,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::net::SocketAddr;

mod models;
mod schema;
mod schools;

use crate::models::{CreateSchool, School};
use crate::schools::get_connection;

const DB_URL: &str = "database.sqlite";

#[tokio::main]
async fn main() {

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Could not run migrations");
    }

    run_db_migrations(&mut get_connection(DB_URL));

    let app = Router::new()
        .route("/school/:id", get(get_school))
        .route("/school", post(create_school));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_school(
    Path(school_id): Path<i32>,
) -> (StatusCode, Json<School>) {

    let school = School {
        id: school_id,
        name: String::from("name")
    };

    (StatusCode::OK, Json(school))
}

async fn create_school(
    Json(payload): Json<CreateSchool>,
) -> Result<Json<School>, StatusCode> {
    schools::create_school(DB_URL, &payload)
        .map(|s| Json(s))
        .map_err(|_| StatusCode::BAD_REQUEST)
}

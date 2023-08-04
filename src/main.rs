use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    response::{IntoResponse},
    extract::{State, Path},
};
use std::net::SocketAddr;
use std::string::ToString;

mod models;
mod schema;
mod database;
mod test_utils;

use crate::models::{CreateSchool, School, RouteDefinition, APIError, APIErrorResponse, AppConfig};
use crate::database::{run_migrations};

const DB_URL: &str = "database.sqlite";

#[tokio::main]
async fn main() {

    run_migrations(DB_URL);

    let config: AppConfig = AppConfig {
        db_url:  "database.sqlite".to_string()
    };

    let app = app(config);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app(config: AppConfig) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/school/", get(get_schools))
        .route("/school/:id", get(get_school))
        .route("/school/", post(create_school))
        .fallback(handler_404)
        .with_state(config)
}

async fn root() -> (StatusCode, Json<Vec<RouteDefinition>>) {
    (StatusCode::OK, Json(vec![
        RouteDefinition {
            url: "/school/".to_string(),
            method: "GET".to_string(),
        },
        RouteDefinition {
            url: "/school/:id".to_string(),
            method: "GET".to_string(),
        },
        RouteDefinition {
            url: "/school/".to_string(),
            method: "POST".to_string(),
        }]))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("Route not found"))
}

async fn get_schools(State(config): State<AppConfig>) -> Result<Json<Vec<School>>, APIErrorResponse> {
    database::get_schools(&config.db_url)
        .map(|s| Json(s))
        .map_err(APIError::from)
        .map_err(|e| (e.status_code, Json(e.error)))
}

async fn get_school(State(config): State<AppConfig>,
                    Path(school_id): Path<i32>) -> Result<Json<School>, APIErrorResponse> {
    database::get_school(&config.db_url, school_id)
        .map(|s| Json(s))
        .map_err(APIError::from)
        .map_err(|e| (e.status_code, Json(e.error)))
}

async fn create_school(State(config): State<AppConfig>,
                       Json(payload): Json<CreateSchool>) -> Result<Json<School>, APIErrorResponse> {
    database::create_school(&config.db_url, &payload)
        .map(|s| Json(s))
        .map_err(APIError::from)
        .map_err(|e| (e.status_code, Json(e.error)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test_helper::TestClient;
    use crate::models::ErrorDetail;
    use crate::test_utils::test_utils::*;

    #[tokio::test]
    async fn can_get_index() {
        let db = TestDatabase::new();
        let app = app(AppConfig { db_url: db.url });
        let client = TestClient::new(app);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<Vec<RouteDefinition>>().await.len(), 3)
    }

    #[tokio::test]
    async fn can_handle_404() {
        let db = TestDatabase::new();
        let app = app(AppConfig { db_url: db.url });
        let client = TestClient::new(app);
        let res = client.get("/badurl").send().await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(res.json::<String>().await, "Route not found")
    }

    #[tokio::test]
    async fn can_create_school() {
        let db = TestDatabase::new();
        let app = app(AppConfig { db_url: db.url });
        let client = TestClient::new(app);
        let new_school = CreateSchool {
            name: "Newbie High".to_string()
        };
        let res = client.post("/school/").json(&new_school).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<School>().await.name, "Newbie High");
    }

    #[tokio::test]
    async fn can_list_schools() {
        let db = TestDatabase::new();
        let app = app(AppConfig { db_url: db.url });
        let client = TestClient::new(app);

        // nothing exists yet, should see an empty list
        let res = client.get("/school/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<Vec<School>>().await.len(), 0);

        // create a school
        let new_school = CreateSchool {
            name: "Newbie High".to_string()
        };
        client.post("/school/").json(&new_school).send().await;

        // should now be able to successfully GET
        let res = client.get("/school/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<Vec<School>>().await.len(), 1);
    }

    #[tokio::test]
    async fn can_get_school() {
        let db = TestDatabase::new();
        let app = app(AppConfig { db_url: db.url });
        let client = TestClient::new(app);

        // nothing exists yet, should see a 404
        let res = client.get("/school/1").send().await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(res.json::<ErrorDetail>().await.message, "Resource not found");

        // create a school
        let new_school = CreateSchool {
            name: "Newbie High".to_string()
        };
        client.post("/school/").json(&new_school).send().await;

        // should now be able to successfully GET
        let res = client.get("/school/1").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.json::<School>().await.name, "Newbie High");
    }
}

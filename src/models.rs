use axum::{http::StatusCode, Json};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::schools)]
#[derive(JsonSchema)]
pub struct CreateSchool {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::schools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(JsonSchema)]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RouteDefinition {
    pub url: String,
    pub method: String,
}

#[derive(Clone)]
pub struct AppConfig {
    pub db_url: String
}

pub struct APIError {
    pub error: ErrorDetail,
    pub status_code: StatusCode,
}

#[derive(Serialize, Deserialize)]
#[derive(JsonSchema)]
pub struct ErrorDetail {
    pub message: String,
}

impl From<diesel::result::Error> for APIError {
    fn from(e: diesel::result::Error) -> Self {
        println!("{}", e);
        match e {
            diesel::result::Error::NotFound => APIError {
                error: ErrorDetail { message: "Resource not found".to_string() },
                status_code: StatusCode::NOT_FOUND,
            },
            _ => APIError {
                error: ErrorDetail { message: "Unexpected error".to_string() },
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}

pub type APIErrorResponse = (StatusCode, Json<ErrorDetail>);

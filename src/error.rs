use std::fmt;

use axum::{
    // extract::rejection::JsonRejection,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[allow(non_camel_case_types)]
pub enum Error {
    // reject_json(JsonRejection),
    db_query_error(sqlx::Error),
    out_of_range_offset,
    invalid_offset,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            // Error::reject_json(rejection) => (rejection.status(), rejection.body_text()),
            Error::db_query_error(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error executing the query: {error}"),
            ),
            Error::out_of_range_offset => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "The offset is bigger than the total items".to_string(),
            ),
            Error::invalid_offset => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "The offset is invalid".to_string(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}


#[allow(non_camel_case_types)]
pub enum AppError {
    no_db_url,
    invalid_db_url,
    db_connection_failed
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            AppError::no_db_url => write!(f, "You didn't enter any database URL"),
            AppError::invalid_db_url => write!(f, "The database URL you entered is invalid"),
            AppError::db_connection_failed => write!(f, "Failed to connect to the database"),
        }
    }
}
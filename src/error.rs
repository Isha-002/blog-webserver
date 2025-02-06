use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[allow(non_camel_case_types)]
pub enum Error {
    reject_json(JsonRejection),
    db_query_error(sqlx::Error),
    invalid_offset,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            Error::reject_json(rejection) => (rejection.status(), rejection.body_text()),
            Error::db_query_error(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error executing the query: {error}"),
            ),
            Error::invalid_offset => (
              StatusCode::UNPROCESSABLE_ENTITY,
              format!("The offset is bigger than the total items"),
          )
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}

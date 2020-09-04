use actix_web::{error, error::ResponseError, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error;
use failure::Fail;
use serde::Serialize;
use serde_json::json;

pub enum Actions<T: Serialize> {
    Create(Result<T, Error>),
    ReadMany(Result<Vec<T>, Error>),
    ReadOne(Result<T, Error>),
    Update(Result<T, Error>),
    Delete(Result<T, Error>),
}

#[derive(Fail, Debug)]
enum DemoError {
    #[fail(display = "Internal Server Error")]
    InternalError,
    #[fail(display = "Bad Request")]
    BadClientData,
    #[fail(display = "Not Found")]
    NotFound,
}

impl error::ResponseError for DemoError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
            .into_body()
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            DemoError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            DemoError::BadClientData => StatusCode::BAD_REQUEST,
            DemoError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
/// parse diesel errors and translates into
/// our generic demo API errors
fn parse_err(dsl_error: Error) -> DemoError {
    match dsl_error {
        Error::NotFound => DemoError::NotFound,
        Error::DeserializationError(_) | Error::SerializationError(_) => DemoError::BadClientData,
        _ => DemoError::InternalError,
    }
}

/// handle building actix http response based on success/failure and ccontent type
fn build_response<T: Serialize>(result: Result<T, Error>, is_json: bool) -> HttpResponse {
    match result {
        Ok(data) => {
            if is_json {
                HttpResponse::build(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json!(data))
                    .into_body()
            } else {
                HttpResponse::build(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                    .body("Success")
                    .into_body()
            }
        }
        Err(dsl_error) => {
            let err: DemoError = parse_err(dsl_error);
            err.error_response()
        }
    }
}

/// handle CRUD actions and call build handler accordingly
/// implements a generic type as long as that type implements
/// the Serialize trait from serde. This is because all data will be run through
/// the serde json! macro that will serialize our rust structs into json.
/// If we called this function with a type that did not implement this trait...
/// BIG CRASH. Compiler driven development ftw.
pub fn handle<T: Serialize>(action: Actions<T>) -> HttpResponse {
    match action {
        Actions::Create(result) | Actions::Update(result) | Actions::ReadOne(result) => {
            build_response(result, true)
        }
        // handle separately incase we want to do pagination or something of the like
        Actions::ReadMany(result) => build_response(result, true),
        Actions::Delete(result) => build_response(result, false),
    }
}

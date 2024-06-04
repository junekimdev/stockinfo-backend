use super::serialize;
use crate::model::ErrorRes;
use actix_web::http::StatusCode;
use tracing::{event, Level};

#[derive(Debug)]
pub enum Error {
    General(String),

    // std errors
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    ParseInt(std::num::ParseIntError),
    TryFromInt(std::num::TryFromIntError),
    Utf8(std::str::Utf8Error),
    FromUtf8(std::string::FromUtf8Error),
    SystemTime(std::time::SystemTimeError),
    Env(std::env::VarError),

    // 3rd party errors
    TokioJoin(tokio::task::JoinError),
    Config(config::ConfigError),
    SerdeJson(serde_json::Error),
    PostgresPool(deadpool_postgres::PoolError),
    Postgres(tokio_postgres::Error),
    RedisPool(deadpool_redis::PoolError),
    Redis(deadpool_redis::redis::RedisError),
    Reqwest(reqwest::Error),
    TimeFormat(time::error::Format),
    TimeParse(time::error::Parse),
    TimeComponentRange(time::error::ComponentRange),
    Zip(zip::result::ZipError),
    XmltreeParse(roxmltree::Error),
    Decimal(rust_decimal::Error),

    // HTTP Error responses
    E400BadRequest(String),
    #[allow(unused)]
    E401Unauthorized(String),
    #[allow(unused)]
    E403Forbidden(String),
    E404NotFound(String),
    #[allow(unused)]
    E409Conflict(String),
    #[allow(unused)]
    E410Gone(String),
    #[allow(unused)]
    E500(String),
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            General(s) => write!(f, "Error: {}", s),

            Io(e) => e.fmt(f),
            Fmt(e) => e.fmt(f),
            ParseInt(e) => e.fmt(f),
            TryFromInt(e) => e.fmt(f),
            Utf8(e) => e.fmt(f),
            FromUtf8(e) => e.fmt(f),
            SystemTime(e) => e.fmt(f),
            Env(e) => e.fmt(f),

            TokioJoin(e) => e.fmt(f),
            Config(e) => e.fmt(f),
            SerdeJson(e) => e.fmt(f),
            PostgresPool(e) => e.fmt(f),
            Postgres(e) => e.fmt(f),
            RedisPool(e) => e.fmt(f),
            Redis(e) => e.fmt(f),
            Reqwest(e) => e.fmt(f),
            TimeFormat(e) => e.fmt(f),
            TimeParse(e) => e.fmt(f),
            TimeComponentRange(e) => e.fmt(f),
            Zip(e) => e.fmt(f),
            XmltreeParse(e) => e.fmt(f),
            Decimal(e) => e.fmt(f),

            E400BadRequest(s) => write!(f, "BAD_REQUEST {}", s),
            E401Unauthorized(s) => write!(f, "UNAUTHORIZED {}", s),
            E403Forbidden(s) => write!(f, "FORBIDDEN {}", s),
            E404NotFound(s) => write!(f, "NOT_FOUND {}", s),
            E409Conflict(s) => write!(f, "CONFLICT {}", s),
            E410Gone(s) => write!(f, "GONE {}", s),
            E500(s) => write!(f, "INTERNAL_SERVER_ERROR {}", s),
        }
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            E400BadRequest(_) => StatusCode::BAD_REQUEST,
            E401Unauthorized(_) => StatusCode::UNAUTHORIZED,
            E403Forbidden(_) => StatusCode::FORBIDDEN,
            E404NotFound(_) => StatusCode::NOT_FOUND,
            E409Conflict(_) => StatusCode::CONFLICT,
            E410Gone(_) => StatusCode::GONE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let message = self.to_string();
        let code = self.status_code();
        event!(Level::ERROR, "{}", message);

        let body = serialize(&ErrorRes {
            code: code.as_u16(),
            message,
        })
        .unwrap();

        match self {
            Error::E401Unauthorized(_) => actix_web::HttpResponse::build(code)
                .insert_header(actix_web::http::header::ContentType::json())
                .insert_header((
                    actix_web::http::header::WWW_AUTHENTICATE,
                    r#"Bearer realm="medicord", error="invalid_token""#,
                ))
                .body(body),
            _ => actix_web::HttpResponse::build(code)
                .insert_header(actix_web::http::header::ContentType::json())
                .body(body),
        }
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::General(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::General(err.to_owned())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::TryFromInt(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(err)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Self::SystemTime(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Env(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::TokioJoin(err)
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Self::Config(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::PostgresPool(err)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::Postgres(err)
    }
}

impl From<deadpool_redis::PoolError> for Error {
    fn from(err: deadpool_redis::PoolError) -> Self {
        Self::RedisPool(err)
    }
}

impl From<deadpool_redis::redis::RedisError> for Error {
    fn from(err: deadpool_redis::redis::RedisError) -> Self {
        Self::Redis(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<time::error::Format> for Error {
    fn from(err: time::error::Format) -> Self {
        Self::TimeFormat(err)
    }
}

impl From<time::error::Parse> for Error {
    fn from(err: time::error::Parse) -> Self {
        Self::TimeParse(err)
    }
}

impl From<time::error::ComponentRange> for Error {
    fn from(err: time::error::ComponentRange) -> Self {
        Self::TimeComponentRange(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Self::Zip(err)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(err: roxmltree::Error) -> Self {
        Self::XmltreeParse(err)
    }
}

impl From<rust_decimal::Error> for Error {
    fn from(err: rust_decimal::Error) -> Self {
        Self::Decimal(err)
    }
}

use serde::Serialize;
use derive_more::Display;
use actix_session::{SessionInsertError, SessionGetError};
use actix_web::{http::StatusCode, HttpResponse, error::BlockingError};

use deadpool_postgres::PoolError;

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

// impl From<MError> for Error {
//     fn from(e: MError) -> Self {
//         Error {
//             status: StatusCode::INTERNAL_SERVER_ERROR,
//             message: e.to_string(),
//         }
//     }
// }

// impl From<anyhow::Error> for Error {
//     fn from(e: anyhow::Error) -> Self {
//         Error {
//             status: StatusCode::INTERNAL_SERVER_ERROR,
//             message: e.to_string(),
//         }
//     }
// }

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<BlockingError> for Error {
    fn from(e: BlockingError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}


impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<SessionInsertError> for Error {
    fn from(e: SessionInsertError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<SessionGetError> for Error {
    fn from(e: SessionGetError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

//
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e,
        }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<scylla::transport::errors::QueryError> for Error {
    fn from(e: scylla::transport::errors::QueryError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<uuid::Error> for Error {
    fn from(e: uuid::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<scylla::cql_to_rust::FromRowError> for Error {
    fn from(e: scylla::cql_to_rust::FromRowError) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

impl From<lily_types::Error> for Error {
    fn from(e: lily_types::Error) -> Self {
        Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        }
    }
}

#[derive(Display, Debug)]
#[display(fmt = "status: {}", status)]
pub struct Error {
    status: StatusCode,
    message: String,
}

impl Error {
    pub fn get_status(&self) -> StatusCode {
        self.status
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    message: String,
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.get_status()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            status: self.status_code().as_u16(),
            message: self.get_message()
        })
    }
}
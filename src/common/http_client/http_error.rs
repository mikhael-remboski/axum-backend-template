use std::error::Error;

pub struct HttpError {
    pub status_code: u16,
    pub message: String,
}

impl HttpError {
    pub fn from_reqwest_err(error: reqwest_middleware::Error) -> Self {
        match error {
            reqwest_middleware::Error::Middleware(_) => HttpError {
                status_code: 500,
                message: "Middleware error".to_string(),
            },
            reqwest_middleware::Error::Reqwest(e) => HttpError {
                status_code: e.status().map_or(500, |s| s.as_u16()),
                message: e.to_string(),
            },
        }
    }

    pub fn from_error(error: Box<dyn Error>) -> Self {
        match error {
            _ => HttpError {
                status_code: 500,
                message: error.to_string(),
            },
        }
    }
}

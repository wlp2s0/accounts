use poem_openapi::{payload::Json, ApiResponse, Object};

use super::User;

// Requests
#[derive(Object)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Object)]
pub struct Pong {
    pub message: String,
}

#[derive(Object)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Responses
#[derive(ApiResponse)]
pub enum PongResponse {
    #[oai(status = 200)]
    Ok(Json<Pong>),
}

#[derive(Object)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn new_json(message: &str) -> Json<Self> {
        Json(Self::new(message))
    }
}

/// Signup responses
#[derive(ApiResponse)]
pub enum SignupResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
}

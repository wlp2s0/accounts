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

/// Signup responses
#[derive(ApiResponse)]
pub enum SignupResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
}

use poem::{IntoResponse, Result};
use poem_openapi::{
    payload::{Json, ParsePayload},
    types::{ParseFromJSON, ToJSON, Type},
    ApiResponse, Object,
};

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
    pub message: String,
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

// impl<T: Serialize> Into<ApiResponse> for T {}

// impl Into<SignupResponse> for User {
//     fn into(self) -> SignupResponse {
//         SignupResponse::Ok(Json(self))
//     }
// }

#[derive(Object)]
pub struct ResponseObject<T: std::marker::Sync + std::marker::Send + ToJSON + ParseFromJSON> {
    pub data: T,
}

#[derive(ApiResponse)]
pub enum SomeResponse<T: std::marker::Send + poem_openapi::types::ToJSON + ParseFromJSON> {
    #[oai(status = 200)]
    Ok(Json<ResponseObject<T>>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 401)]
    Unauthorized,
}

// enum Errors {
//     NotFound(String),
// }

impl<T: ToJSON + ParseFromJSON> Into<Result<SomeResponse<T>>> for ResponseObject<T> {
    fn into(self) -> Result<SomeResponse<T>> {
        Ok(SomeResponse::Ok(Json(self)))
    }
}

impl<T: ToJSON + ParseFromJSON> Into<Result<SomeResponse<T>>> for ErrorResponse {
    fn into(self) -> Result<SomeResponse<T>> {
        Ok(SomeResponse::NotFound(Json(self)))
    }
}

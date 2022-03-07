use argon2::{self, Config};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Database,
};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, Tags,
};
use serde::{Deserialize, Serialize};

use crate::utils::db::get_db;

#[derive(Object)]
pub struct SignupRequest {
    email: String,
    password: String,
}

#[derive(ApiResponse)]
pub enum SignupResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
}

// #[derive(ApiResponse)]
// pub struct LoginRequest {
//     email: String,
//     password: String,
// }

#[derive(Debug, Object, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    email: String,
    password: String,
}

#[derive(Object)]
pub struct Pong {
    message: String,
}

#[derive(ApiResponse)]
pub enum PongResponse {
    #[oai(status = 200)]
    Ok(Json<Pong>),
}

pub struct AuthApi;

#[OpenApi]
impl AuthApi {
    #[oai(path = "/ping", method = "get")]
    pub async fn index(&self) -> PongResponse {
        PongResponse::Ok(Json(Pong {
            message: "pong".to_string(),
        }))
    }

    #[oai(path = "/signup", method = "post")]
    pub async fn signup(&self, user: Json<SignupRequest>) -> SignupResponse {
        let db = get_db().await;
        let collection = db.collection::<User>("users");

        let SignupRequest { email, password } = user.0;
        let mut user: User = User {
            email,
            password,
            id: None,
        };
        let salt = b"randomsalt";
        let config = Config::default();

        user.password = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();
        println!("{user:?}");
        let id = match collection.insert_one(&user, None).await {
            Ok(result) => result.inserted_id,
            Err(error) => panic!("{error:?}"),
        };
        user.id = id.as_object_id();

        SignupResponse::Ok(Json(user))
    }

    // pub async fn login(
    //     data: web::Data<Database>,
    //     login: web::Json<LoginRequest>,
    // ) -> Result<Json<User>, ()> {
    //     let db = &data;
    //     let collection = db.collection::<User>("users");
    //     let filter = doc! { "email": login.email.to_string() };
    //     let user = match collection.find_one(filter, None).await {
    //         Ok(user_model) => {
    //             let found_user = user_model.unwrap();
    //             let matches =
    //                 argon2::verify_encoded(&found_user.password, login.password.as_bytes()).unwrap();
    //             found_user
    //         }
    //         Err(error) => panic!("{error:?}"),
    //     };
    //     Ok(Json(user))
    // }
}

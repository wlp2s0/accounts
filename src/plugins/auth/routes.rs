use argon2::{self, Config};

use mongodb::bson::doc;
use poem::{error::InternalServerError, session::Session, Result};
use poem_openapi::{payload::Json, OpenApi};

use crate::{plugins::auth::User, utils::db::get_db};

use super::{ErrorResponse, LoginRequest, Pong, PongResponse, SignupRequest, SignupResponse};

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

    #[oai(path = "/login", method = "post")]
    pub async fn login(
        &self,
        login: Json<LoginRequest>,
        session: &Session,
    ) -> Result<SignupResponse> {
        let db = get_db().await;
        let collection = db.collection::<User>("users");
        let filter = doc! { "email": login.email.to_string() };
        let user = collection
            .find_one(filter, None)
            .await
            .map_err(InternalServerError)?;
        match user {
            Some(user) => {
                let matches = argon2::verify_encoded(&user.password, login.password.as_bytes())
                    .map_err(InternalServerError)?;
                if !matches {
                    return Ok(SignupResponse::NotFound(ErrorResponse::new_json(
                        "User not found",
                    )));
                }
                session.set("user", &user);
                Ok(SignupResponse::Ok(Json(user)))
            }
            None => Ok(SignupResponse::NotFound(ErrorResponse::new_json(
                "User not found",
            ))),
        }
    }

    #[oai(path = "/am-i-auth", method = "post")]
    pub async fn am_i_auth(&self, session: &Session) -> Result<SignupResponse> {
        // match
        // let user: Option<User> = session.get("user");
        match session.get("user") {
            Some(user) => Ok(SignupResponse::Ok(Json(user))),
            None => Ok(SignupResponse::NotFound(ErrorResponse::new_json(
                "User not found",
            ))),
        }
    }
}

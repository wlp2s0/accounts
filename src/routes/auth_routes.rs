use poem_openapi::{
	payload::{Json, PlainText},
	ApiResponse, OpenApi, Tags,
};
use argon2::{self, Config};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Database,
};


// #[derive(ApiResponse)]
// pub struct SignupRequest {
//     email: String,
//     password: String,
// }

// #[derive(ApiResponse)]
// pub struct LoginRequest {
//     email: String,
//     password: String,
// }

// #[derive(ApiResponse)]
// struct User {
//     #[serde(
//         rename = "_id",
//         skip_serializing_if = "Option::is_none",
//         serialize_with = "serialize_object_id"
//     )]
//     id: Option<ObjectId>,
//     email: String,
//     password: String,
// }

// impl From<SignupRequest> for User {
//     fn from(request: SignupRequest) -> Self {
//         let SignupRequest { email, password } = request;
//         Self {
//             email,
//             password,
//             ..Default::default()
//         }
//     }
// }

#[derive(ApiResponse)]
enum PongResponse {
	#[oai(status = 200)]
    Ok,
}

pub struct AuthApi;

#[OpenApi]
impl AuthApi {
    #[oai(path = "/ping", method = "get")]
    pub async fn index(&self) -> PongResponse {
        PongResponse::Ok
    }

    // #[oai(path = "/signup", method = "post")]
    // pub async fn signup(
    //     data: web::Data<Database>,
    //     user: web::Json<SignupRequest>,
    // ) -> Result<String, ()> {
    //     let db = &data;
    //     let collection = db.collection::<User>("users");
    
    //     let mut user: User = user.into_inner().into();
    //     let salt = b"randomsalt";
    //     let config = Config::default();
    
    //     user.password = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();
    //     println!("{user:?}");
    //     let id = match collection.insert_one(user, None).await {
    //         Ok(result) => result.inserted_id,
    //         Err(error) => panic!("{error:?}"),
    //     };
    
    //     Ok(id.to_string())
    // }
    
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




use crate::utils::serializers::serialize_object_id;
use argon2::{self, Config};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Database,
};
use paperclip::actix::{
    api_v2_operation,
    delete,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    get,
    post,
    put,
    // use this instead of actix_web::web
    web::{self, Json},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct SignupRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Apiv2Schema)]
struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    id: Option<ObjectId>,
    email: String,
    password: String,
}

impl From<SignupRequest> for User {
    fn from(request: SignupRequest) -> Self {
        let SignupRequest { email, password } = request;
        Self {
            email,
            password,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct PongResponse {
    message: String,
}

#[api_v2_operation]
#[get("/ping")]
pub async fn ping() -> Result<Json<PongResponse>, ()> {
    // let db = &data;
    // let collection = db.collection::<Document>("books");
    // let docs = vec![
    //     doc! { "title": "1984", "author": "George Orwell" },
    //     doc! { "title": "Animal Farm", "author": "George Orwell" },
    //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    // ];

    // // Insert some documents into the "mydb.books" collection.
    // collection.insert_many(docs, None).await.expect("asdasd");

    // HttpResponse::Ok().body("pong")
    Ok(Json(PongResponse {
        message: "pong".to_string(),
    }))
}

#[api_v2_operation]
#[post("/signup")]
pub async fn signup(
    data: web::Data<Database>,
    user: web::Json<SignupRequest>,
) -> Result<String, ()> {
    let db = &data;
    let collection = db.collection::<User>("users");

    let mut user: User = user.into_inner().into();
    let salt = b"randomsalt";
    let config = Config::default();

    user.password = argon2::hash_encoded(user.password.as_bytes(), salt, &config).unwrap();
    println!("{user:?}");
    let id = match collection.insert_one(user, None).await {
        Ok(result) => result.inserted_id,
        Err(error) => panic!("{error:?}"),
    };

    Ok(id.to_string())
}

#[api_v2_operation]
#[post("/login")]
pub async fn login(
    data: web::Data<Database>,
    login: web::Json<LoginRequest>,
) -> Result<Json<User>, ()> {
    let db = &data;
    let collection = db.collection::<User>("users");
    let filter = doc! { "email": login.email.to_string() };
    let user = match collection.find_one(filter, None).await {
        Ok(user_model) => {
            let found_user = user_model.unwrap();
            let matches =
                argon2::verify_encoded(&found_user.password, login.password.as_bytes()).unwrap();
            found_user
        }
        Err(error) => panic!("{error:?}"),
    };
    Ok(Json(user))
}

use mongodb::bson::oid::ObjectId;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
}

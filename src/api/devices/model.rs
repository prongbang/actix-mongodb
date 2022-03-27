use mongodb::bson;
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub flag: bool,
    // #[serde(with = "serde_with::json::nested")]
    // b: B,
}

#[derive(Deserialize)]
pub struct Params {
    pub id: String,
}

impl Device {
    pub fn to_doc(&self) -> Document {
        doc! {
            "name": self.name.clone(),
            "flag": self.flag.clone(),
        }
    }
}
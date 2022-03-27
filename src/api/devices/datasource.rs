use std::str::FromStr;
use std::sync::Arc;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use crate::api::devices::model::Device;

const DB_NAME: &str = "iotDB";
const COLL_NAME: &str = "device";

pub trait DataSource {
    fn get_list(&self) -> Vec<Device>;
    fn get_by_id(&self, id: String) -> Result<Device, String>;
    fn create(&self, data: &Device) -> Result<Device, String>;
    fn update_by_id(&self, data: &Device) -> Result<Device, String>;
    fn delete_by_id(&self, id: String) -> bool;
}

pub struct DeviceDataSource {
    mongodb: mongodb::sync::Client,
}

impl DeviceDataSource {
    pub fn new(mongodb: mongodb::sync::Client) -> Arc<dyn DataSource> {
        Arc::new(DeviceDataSource {
            mongodb
        })
    }
}

// Example: https://ichi.pro/th/ca-srang-seirfwexr-rest-api-dwy-rust-laea-mongodb-di-xyangri-144476358523348
impl DataSource for DeviceDataSource {
    fn get_list(&self) -> Vec<Device> {
        todo!()
    }

    fn get_by_id(&self, id: String) -> Result<Device, String> {
        todo!()
    }

    fn create(&self, data: &Device) -> Result<Device, String> {
        let mut device = data.clone();
        let db = self.mongodb.database(DB_NAME);
        let collection = db.collection::<Device>(COLL_NAME);

        let result = collection.insert_one(data.clone(), None);
        return match result {
            Ok(rs) => {
                device.id = rs.inserted_id.as_object_id();
                Ok(device.clone())
            }
            Err(_) => Err("Cannot add data".to_string())
        };
    }

    fn update_by_id(&self, data: &Device) -> Result<Device, String> {
        let db = self.mongodb.database(DB_NAME);
        let collection = db.collection::<Device>(COLL_NAME);

        let device = data.clone();
        let query = doc! {"_id": device.id.unwrap()};
        let update = doc! {"$set": device.to_doc()};

        let result = collection.update_one(query, update, None);
        match result {
            Ok(_) => {
                Ok(device.clone())
            }
            Err(_) => Err("Cannot update data".to_string())
        }
    }

    fn delete_by_id(&self, id: String) -> bool {
        let db = self.mongodb.database(DB_NAME);
        let collection = db.collection::<Device>(COLL_NAME);

        let id = ObjectId::from_str(id.as_str()).unwrap();
        let query = doc! {"_id": id};

        let result = collection.delete_one(query, None);
        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use crate::api::devices::model::Device;

const DB_NAME: &str = "iotDB";
const COLL_NAME: &str = "device";

pub trait DataSource {
    fn get_list(&self) -> Vec<Device>;
    fn get_by_id(&self, id: String) -> Device;
    fn create(&self, data: &Device) -> Result<Device, &str>;
    fn update_by_id(&self, id: String, data: &Device) -> bool;
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

impl DataSource for DeviceDataSource {
    fn get_list(&self) -> Vec<Device> {
        todo!()
    }

    fn get_by_id(&self, id: String) -> Device {
        todo!()
    }

    fn create(&self, data: &Device) -> Result<Device, &str> {
        let mut device = data.clone();
        let db = self.mongodb.database(DB_NAME);
        let collection = db.collection::<Device>(COLL_NAME);
        let result = collection.insert_one(data.clone(), None);
        return match result {
            Ok(rs) => {
                device.id = rs.inserted_id.as_object_id();
                Ok(device)
            }
            Err(_) => Err("Cannot add data")
        };
    }

    fn update_by_id(&self, id: String, data: &Device) -> bool {
        todo!()
    }

    fn delete_by_id(&self, id: String) -> bool {
        todo!()
    }
}
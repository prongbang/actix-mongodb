use std::sync::Arc;
use crate::api::devices;
use crate::api::devices::datasource::DeviceDataSource;
use crate::api::devices::repository::DeviceRepository;
use crate::database;

pub struct Container {
    pub device_repo: Arc<dyn devices::repository::Repository>,
}

pub fn inject() -> Container {
    let mongodb = database::mongodb::new();
    let device_ds = DeviceDataSource::new(mongodb);
    let device_repo = DeviceRepository::new(device_ds.clone());

    Container {
        device_repo: device_repo.clone()
    }
}
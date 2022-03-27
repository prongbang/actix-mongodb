use std::sync::Arc;
use crate::api::devices;
use crate::api::devices::datasource::DeviceDataSource;
use crate::database;

pub struct Container {
    pub device_ds: Arc<dyn devices::datasource::DataSource>,
}

pub fn inject() -> Container {
    let mongodb = database::mongodb::new();
    let device_ds = DeviceDataSource::new(mongodb);

    Container {
        device_ds: device_ds.clone()
    }
}
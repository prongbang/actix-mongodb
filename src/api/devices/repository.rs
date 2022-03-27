use std::sync::Arc;
use crate::api::devices::datasource::DataSource;
use crate::api::devices::model::Device;

pub trait Repository {
    fn get_list(&self) -> Vec<Device>;
    fn get_by_id(&self, id: String) -> Result<Device, String>;
    fn create(&self, data: &Device) -> Result<Device, String>;
    fn update_by_id(&self, data: &Device) -> Result<Device, String>;
    fn delete_by_id(&self, id: String) -> bool;
}

pub struct DeviceRepository {
    pub datasource: Arc<dyn DataSource>,
}

impl DeviceRepository {
    pub fn new(datasource: Arc<dyn DataSource>) -> Arc<dyn Repository> {
        Arc::new(DeviceRepository { datasource })
    }
}

impl Repository for DeviceRepository {
    fn get_list(&self) -> Vec<Device> {
        self.datasource.get_list()
    }

    fn get_by_id(&self, id: String) -> Result<Device, String> {
        self.datasource.get_by_id(id)
    }

    fn create(&self, data: &Device) -> Result<Device, String> {
        self.datasource.create(&data)
    }

    fn update_by_id(&self, data: &Device) -> Result<Device, String> {
        self.datasource.update_by_id(&data)
    }

    fn delete_by_id(&self, id: String) -> bool {
        self.datasource.delete_by_id(id)
    }
}


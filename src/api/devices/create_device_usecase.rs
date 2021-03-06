use std::sync::Arc;
use crate::api::devices::model::Device;
use crate::api::devices::repository::Repository;

pub fn execute(device: &Device, repository: &Arc<dyn Repository>) -> Result<Device, String> {
    repository.create(&device)
}
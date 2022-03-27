use std::sync::Arc;
use crate::api::devices::model::Device;
use crate::api::devices::repository::Repository;

pub fn execute(id: String, repository: &Arc<dyn Repository>) -> Result<Device, String> {
    repository.get_by_id(id)
}
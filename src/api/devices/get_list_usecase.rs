use std::sync::Arc;
use crate::api::devices::model::Device;
use crate::api::devices::repository::Repository;

pub fn execute(repository: &Arc<dyn Repository>) -> Vec<Device> {
    repository.get_list()
}
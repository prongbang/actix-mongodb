use std::sync::Arc;
use crate::api::devices::repository::Repository;

pub fn execute(id: String, repository: &Arc<dyn Repository>) -> bool {
    repository.delete_by_id(id)
}
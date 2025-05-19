use ulid::Ulid;

use crate::Aggregate;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

pub trait Repository<T: Aggregate<T> + Default> {
    fn create(&self, aggregate: T) -> Result<(), RepositoryError>;
    fn update(&self, aggregate: T) -> Result<(), RepositoryError>;
    fn delete(&self, id: Ulid) -> Result<(), RepositoryError>;
    fn get(&self, id: Ulid) -> Result<T, RepositoryError>;
}

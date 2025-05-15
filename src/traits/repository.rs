use ulid::Ulid;

use crate::Aggregate;

pub trait Repository<T: Aggregate<T> + Default> {
    fn create(&self, aggregate: T) -> Result<(), String>;
    fn update(&self, aggregate: T) -> Result<(), String>;
    fn delete(&self, id: Ulid) -> Result<(), String>;
    fn get(&self, id: Ulid) -> Result<T, String>;
}
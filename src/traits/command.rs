pub trait Command<T, E, Err>
where
    Err: std::error::Error + Send + Sync + 'static,
{
    fn execute(&self, state: T) -> Result<Vec<E>, Err>;
}

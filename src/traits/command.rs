pub trait Command<T, E> {
    fn execute(&self, state: T) -> Result<Vec<E>, String>;
}
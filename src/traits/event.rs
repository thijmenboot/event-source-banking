
pub trait Event<T> {
    fn apply(&self, state: T) -> Result<T, String>;
}

// impl<T: Default> Event<T> for T {
//     fn apply(&self, state: T) -> Result<T, String> {
//         Ok(state)
//     }
// }
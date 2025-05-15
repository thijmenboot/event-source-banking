use super::{Command, Event};

pub trait Aggregate<T: Default> {
    fn from_history<E: Event<T>>(history: Vec<E>) -> Result<T, String>;
    fn handle_command<C: Command<T, E>, E: Event<T>>(state: T, command: C) -> Result<Vec<E>, String>;
    fn handle_event<E: Event<T>>(state: T, event: E) -> Result<T, String>;
}

impl<T: Default + 'static> Aggregate<T> for T {
    fn from_history<E: Event<T>>(history: Vec<E>) -> Result<T, String> {
        let mut state = T::default();
        for event in history {
            state = Self::handle_event(state, event)?;
        }
        Ok(state)
    }
    
    fn handle_command<C: Command<T, E>, E: Event<T>>(state: T, command: C) -> Result<Vec<E>, String> {
        command.execute(state)
    }
    
    fn handle_event<E: Event<T>>(state: T, event: E) -> Result<T, String> {
        event.apply(state)
    }
}
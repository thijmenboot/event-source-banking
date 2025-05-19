use super::{Command, Event, event::ApplyError};

pub trait Aggregate<T: Default> {
    fn from_history<E: Event<T>>(history: Vec<E>) -> Result<T, ApplyError>;
    fn handle_command<C, E, Err>(state: T, command: C) -> Result<Vec<E>, Err>
    where
        C: Command<T, E, Err>,
        E: Event<T>,
        Err: std::error::Error + Send + Sync + 'static;
    fn handle_event<E: Event<T>>(state: &mut T, event: E) -> Result<(), ApplyError>;
}

impl<T: Default + 'static> Aggregate<T> for T {
    fn from_history<E: Event<T>>(history: Vec<E>) -> Result<T, ApplyError> {
        let mut state = T::default();
        for event in history {
            Self::handle_event(&mut state, event)?;
        }
        Ok(state)
    }

    fn handle_command<C, E, Err>(state: T, command: C) -> Result<Vec<E>, Err>
    where
        C: Command<T, E, Err>,
        E: Event<T>,
        Err: std::error::Error + Send + Sync + 'static,
    {
        command.execute(state)
    }

    fn handle_event<E: Event<T>>(state: &mut T, event: E) -> Result<(), ApplyError> {
        event.apply(state)
    }
}

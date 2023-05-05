use std::sync::Arc;

pub mod client;
pub mod display;
pub mod game;
pub mod sound;
pub mod texture;

/* This function is not rational */
pub fn wait_unwrap_and_map<T, V, F>(arc: Arc<T>, call: F) -> V
    where F: FnOnce(T) -> V
{
    let x = Arc::try_unwrap(arc);
    match x {
        Ok(x) => {
            call(x)
        }
        Err(x) => {
            while Arc::strong_count(&x) > 1 {};
            wait_unwrap_and_map(x, call)
        }
    }
}
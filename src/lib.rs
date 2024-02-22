mod app;
pub use app::App;

pub mod captures;
mod colors;
pub mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    capture: String, // TODO: can get rid of it
    start: usize,
    end: usize,
}

mod app;
pub use app::App;

pub mod captures;
mod colors;
pub mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    capture: String,
    start: usize,
    end: usize,
}

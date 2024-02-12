mod app;
pub use app::App;

mod captures;
mod colors;
mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    capture: String,
    start: usize,
    end: usize,
}

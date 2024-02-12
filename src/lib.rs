mod app;
pub use app::App;

mod captures;
mod colors;
mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    capture: String, // TODO: can get rid of it
    start: usize,
    end: usize,
}

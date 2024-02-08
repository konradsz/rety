mod app;
pub use app::App;

mod colors;
mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    start: usize,
    end: usize,
}

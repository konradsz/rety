mod app;
pub use app::App;

pub mod group_captures;
pub mod layout;
pub mod styles;

#[derive(Debug, PartialEq, Eq)]
pub struct MatchGroup {
    name: String,
    capture: String,
    start: usize,
    end: usize,
}

impl MatchGroup {
    pub fn new(name: String, capture: String, start: usize, end: usize) -> Self {
        Self {
            name,
            capture,
            start,
            end,
        }
    }
}

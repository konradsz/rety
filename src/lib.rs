mod app;
pub use app::App;

mod layout;

#[derive(Debug)]
pub struct MatchGroup {
    name: String,
    start: usize,
    end: usize,
}

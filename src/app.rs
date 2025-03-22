use std::error;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub selected_city: usize,
    pub cities: Vec<String>, // or whatever type your cities are
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            selected_city: 0,         // Initialize with default index 0
            cities: Vec::new(),       // Initialize with empty vector
        }
    }
}

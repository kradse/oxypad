use gtk4 as gtk;
use gtk::TextBuffer;

use std::path::PathBuf;

pub struct AppState {
	pub active_index: usize,
    pub taps: Vec<TapState>,
}
impl AppState {
	// Constants
	// Constructors
    pub fn new() -> Self {
        Self {
            active_index: 0,
            taps: vec![TapState::new()],
        }
    }
	// Public functions
	// Private functions
}

pub struct TapState {
	pub path: Option<PathBuf>,
    pub buffer: TextBuffer,
    pub modified: bool,
}
impl TapState {
	// Constants
	// Constructors
    pub fn new() -> Self {
        Self {
            path: None,
            buffer: TextBuffer::new(None),
            modified: false,
        }
    }
	// Public functions
    pub fn title(&self) -> String {
        let filename = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled");

        if self.modified {
            format!("OxyPad - {} *", filename)
        } else {
            format!("OxyPad - {}", filename)
        }
    }
	// Private functions
}

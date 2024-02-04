use gtk::gio;

pub struct Archive {
	pub path: Option<gio::File>,
}

impl Default for Archive {
	fn default() -> Self {
		Archive {
			path: None,
		}
	}
}

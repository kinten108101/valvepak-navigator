use gtk::prelude::*;
use gtk::{glib, gio};

mod application;
#[rustfmt::skip]
mod config;
mod models;
mod main_window;

fn main() -> glib::ExitCode {
	let resources = gio::Resource::load(config::PKGDATADIR.to_owned() + "/valvepak-navigator.gresource")
		.unwrap();
	gio::resources_register(&resources);

	application::application().run()
}

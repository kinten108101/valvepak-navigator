use gtk::{glib, prelude::{ApplicationExt, WidgetExt}, glib::clone};
use crate::main_window::window;

pub fn application() -> adw::Application {
	let app = adw::Application::builder()
		.application_id("com.github.kinten108101.ValvePakNavigator")
		.build();

	app.connect_activate(clone!(@weak app => move |_| {
		let window: gtk::Window = window(&app.try_into().unwrap());
		window.set_visible(true);
	}));

	app
}
use gtk::prelude::*;
use gtk::glib;

mod application;

fn main() -> glib::ExitCode {
	application::application().run()
}
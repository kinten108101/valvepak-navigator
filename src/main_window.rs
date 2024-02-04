use gtk::prelude::*;
use gtk::{gio, gdk, glib};
use crate::config;
use crate::models::archive::Archive;

trait StackLike {
	fn set_visible_page(&self, child_name: &str);
}

impl StackLike for gtk::Stack {
	fn set_visible_page(&self, child_name: &str) {
		self.set_visible_child_name(child_name)
	}
}

struct StackWrapper<'a, T: StackLike>(&'a T);

impl<'a, T: StackLike> StackWrapper<'a, T> {
	fn set_visible_child_name(&self, child_name: &str) {
		self.0.set_visible_page(child_name)
	}
}

pub fn window(application: &gtk::Application) -> gtk::Window {
	let builder = gtk::Builder::new();
	builder
		.add_from_resource(&(config::RESOURCE_PREFIX.to_owned() + "gtk/window.ui"))
		.unwrap();

	let window: adw::ApplicationWindow = builder
		.object("window")
		.unwrap();
	window.set_application(Some(application));

	let window_content_stack: gtk::Stack = builder
		.object("window_content_stack")
		.unwrap();

	let drag_revealer: gtk::Revealer = builder
		.object("drag_revealer")
		.unwrap();

	let target = gtk::DropTargetAsync::builder()
		// FIXME(kinten): Am I owning the object here?
		.formats(&gdk::ContentFormats::for_type(gio::File::static_type()))
		.actions(gdk::DragAction::COPY)
		.build();

	target.connect_drag_enter(
		glib::clone!(@weak drag_revealer, @weak window_content_stack as main_nav_view => @default-panic, move |_, _, _, _| {
			main_nav_view.add_css_class("blurred");
			drag_revealer.set_reveal_child(true);
			gdk::DragAction::COPY
		})
	);

	target.connect_drag_leave(
		glib::clone!(@weak drag_revealer, @weak window_content_stack as main_nav_view => move |_, _| {
			main_nav_view.remove_css_class("blurred");
			drag_revealer.set_reveal_child(false);
		})
	);

	target.connect_drop(
		glib::clone!(@weak window_content_stack => @default-panic, move |_, drop, _, _| {
			let wrap = StackWrapper(&window_content_stack);
			wrap.set_visible_child_name("main");
			drop.read_value_async(gio::File::static_type(), glib::source::Priority::DEFAULT, gio::Cancellable::NONE, |res| {
				match res {
					Err(e) => println!("Oh no!"),
					Ok(value) => {
						let file = value.get::<gio::File>().unwrap();
						let path = file.path().unwrap();
						let content = path.to_str().unwrap();
						println!("file: {}", content);
					}
				}
			});
			true
		})
	);

	window.add_controller(target);

	let item = Archive { path: Some(gio::File::for_path("child")) };

	window.try_into().unwrap()
}

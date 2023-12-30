pub fn application() -> adw::Application {
	let app = adw::Application::builder()
		.application_id("com.github.kinten108101.ValvePakNavigator")
		.build();

	app
}
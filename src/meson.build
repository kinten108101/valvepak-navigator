interface = [
	'gtk/window.blp'
]

interface_files = files(interface)

blueprints = custom_target('blueprints',
	input: interface_files,
	output: '.',
	build_by_default: true,
	build_always_stale: true,
	command: [blueprint_compiler,
		'batch-compile',
		'@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)

UI = ''
foreach x : interface
	UI = ''.join([UI, '<file>'])
	UI = ''.join([UI, x.replace('.blp', '.ui')])
	UI = ''.join([UI, '</file>'])
endforeach

icons = []

ICONS = ''
foreach x : icons
	ICONS = ''.join([ICONS, '<file alias="'])
	ICONS = ''.join([ICONS, x])
	ICONS = ''.join([ICONS, '">icons/'])
	ICONS = ''.join([ICONS, x])
	ICONS = ''.join([ICONS, '</file>'])
endforeach

resources = gnome.compile_resources(
	'valvepak-navigator',
	configure_file(
		input:  'app.gresource.xml.in',
		output: 'valvepak-navigator.gresource.xml',
		configuration: {
			'UI': UI,
			'ICONS': ICONS,
		},
	),
	dependencies: blueprints,
	gresource_bundle: true,
	install: true,
	install_dir: get_option('prefix') / get_option('datadir') / 'com.github.kinten108101.ValvePakNavigator',
)

configure_file(
	input: 'config.rs.in',
	output: 'config.rs',
	configuration: {
		'APPID': 'com.github.kinten108101.ValvePakNavigator',
		'PKGDATADIR': get_option('prefix') / get_option('datadir') / 'com.github.kinten108101.ValvePakNavigator',
		'RESOURCE_PREFIX': '/com/github/kinten108101/ValvePakNavigator/',
	},
)

run_command(
	'cp',
	join_paths(meson.project_build_root(), 'src', 'config.rs'),
	join_paths(meson.project_source_root(), 'src', 'config.rs'),
	check: true
)

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]
cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]

custom_target(
	'cargo-build',
	build_by_default: true,
	build_always_stale: true,
	output: 'valvepak-navigator',
	console: true,
	install: true,
	install_dir: get_option('prefix') / get_option('bindir'),
	depends: [resources],
	command: [
		'env',
		cargo_env,
		cargo, 'build',
		cargo_options,
		'&&',
		'cp', 'src' / 'debug' / meson.project_name(), '@OUTPUT@',
	]
)

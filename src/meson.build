resources = gnome.compile_resources(
  'resources',
  'app.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: get_option('prefix') / get_option('datadir') / 'com.github.kinten108101.ValvePakNavigator',
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
project('valvepak-navigator',
        'rust',
        license: 'GPL-3.0-or-later',
        meson_version: '>= 0.59')

gnome = import('gnome')

dependency('gtk4', version: '>= 4.12.0')
dependency(
  'libadwaita-1', version: '>= 1.4.0',
  fallback: ['libadwaita', 'libadwaita_dep'],
  default_options: ['tests=false', 'examples=false', 'vapi=false']
)

cargo = find_program('cargo', required: true)
blueprint_compiler = find_program('blueprint-compiler', version: '>=0.8.1', required: true)

cargo_version = run_command(cargo, '--version', check: true).stdout().strip()
message(cargo_version)
rustc_version = run_command('rustc', '--version', check: true).stdout().strip()
message(rustc_version)

subdir('data')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)

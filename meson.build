project(
  'cba-midi',
  'rust',
  version: '0.1.0',
  license: 'GPL-3.0',
  meson_version: '>= 0.59'
)


gnome = import('gnome')

base_id = 'eu.muehml.cba-midi'
application_id = base_id

dependency('gtk4', version: '>=4.0.0')
dependency('libadwaita-1', version: '>1.2.0')

blueprint_compiler = find_program('blueprint-compiler', required: true)
cargo = find_program('cargo', required: true)
desktop_file_validate = find_program('desktop-file-validate', required: false)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()

meson.add_dist_script(
  'build-aux/dist-vendor.py'
)



subdir('data')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: false,
  update_desktop_database: false,
)

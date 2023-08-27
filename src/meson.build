subdir('ui')

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

global_conf = configuration_data()
global_conf.set_quoted('PKGDATADIR', pkgdatadir)
global_conf.set_quoted('APP_ID', application_id)

gnome.compile_resources('cba-midi-resources',
  'cba-midi.gresource.xml',
  dependencies: blueprints,
  source_dir: ['src', meson.current_build_dir()],
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir
)

config = configure_file(
  input: 'config.rs.in',
  output: 'config.rs',
  configuration: global_conf
)

run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true
)

rust_target = 'debug'

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
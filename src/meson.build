cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

global_conf = configuration_data()
global_conf.set_quoted('PKGDATADIR', pkgdatadir)
global_conf.set_quoted('APP_ID', application_id)
# without .to_string() booleans will be capitalized
global_conf.set('RELOCATABLE', get_option('relocatable').to_string())

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

if get_option('buildtype') == 'release'
  rust_target = 'release'
  cargo_options += '--release'
else
  rust_target = 'debug'
endif

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]

exe_suffix = ''
if host_machine.system() == 'windows'
  exe_suffix = '.exe'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name() + exe_suffix,
  console: true,
  install: true,
  install_dir: bindir,
  env: cargo_env,
  command: [
    find_program('python3'),
    meson.project_source_root() / 'build-aux' / 'cargo-build.py',
    meson.project_build_root() / 'src' / rust_target / meson.project_name() + exe_suffix,
    '@OUTPUT@',
    cargo_options,
  ]
)

launcher = executable(
  'launcher',
  meson.project_source_root() / 'build-aux' / 'launcher.rs',
  install: get_option('relocatable'),
  install_dir: prefix
)

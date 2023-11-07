subdir('ui')
install_data('map.txt', install_dir: pkgdatadir)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
        '@0@.desktop'.format(base_id)
    ]
  )
endif

desktop_file = install_data(
  '@0@.desktop'.format(base_id),
  install_dir: datadir / 'applications',
)

app_icon = install_data(
  'icons' / 'scalable' / 'apps' / '@0@.svg'.format(base_id),
  install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
)


gnome.compile_resources('cba-midi-resources',
  'cba-midi.gresource.xml',
  dependencies: blueprints,
  source_dir: ['src', meson.current_build_dir()],
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir
)


install_data('map.txt', install_dir: pkgdatadir)

desktop_conf = configuration_data()
desktop_conf.set('BINDIR', bindir)

desktop_file = configure_file(
  input: '@0@.desktop.in'.format(base_id),
  output: '@BASENAME@',
  configuration: desktop_conf,
  install: true,
  install_dir: datadir / 'applications',
)

if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file
    ]
  )
endif

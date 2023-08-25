
blueprint_files = files('window.blp')

blueprint_ui = []
foreach blp : blueprint_files
  blueprint_ui += '@0@'.format(blp).replace('.blp', '.ui').split('/')[-1]
endforeach

blueprints = custom_target('blueprints',
  input: blueprint_files,
  output: blueprint_ui,
  command: [
    blueprint_compiler,
    'batch-compile',
    '@OUTDIR@',
    '@CURRENT_SOURCE_DIR@',
    '@INPUT@'
  ]
)
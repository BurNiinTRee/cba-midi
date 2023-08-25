{
  alsaLib,
  autoPatchelfHook,
  blueprint-compiler,
  cairo,
  cargo,
  gdk-pixbuf,
  glib,
  graphene,
  gtk4,
  harfbuzz,
  libjack2,
  meson,
  mold,
  ninja,
  pango,
  pkg-config,
  rustc,
  rustPlatform,
  stdenv,
  wrapGAppsHook4
}:
stdenv.mkDerivation (final: {
  name = "cba-midi";
  src = ../.;
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ../Cargo.lock;
  };
  nativeBuildInputs = [
    autoPatchelfHook
    blueprint-compiler
    cargo
    mold
    pkg-config
    meson
    ninja
    rustc
    rustPlatform.cargoSetupHook
    wrapGAppsHook4
  ];
  buildInputs = [
    alsaLib
    cairo
    gdk-pixbuf
    glib
    graphene
    gtk4
    harfbuzz
    libjack2
    pango
  ];
})

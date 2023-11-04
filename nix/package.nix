{
  lib,
  alsaLib,
  autoPatchelfHook,
  blueprint-compiler,
  cairo,
  cargo,
  desktop-file-utils,
  gdk-pixbuf,
  glib,
  graphene,
  gtk4,
  harfbuzz,
  libadwaita,
  libjack2,
  meson,
  mold,
  ninja,
  pango,
  pkg-config,
  rustPlatform,
  rustc,
  stdenv,
  wrapGAppsHook4,
}:
stdenv.mkDerivation (final: {
  name = "cba-midi";
  src = lib.fileset.toSource {
    root = ./..;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../src
      ../data
      ../meson.build
      ../build-aux/dist-vendor.py
      ../build-aux/cargo-build.py
    ];
  };
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ../Cargo.lock;
  };
  CARGO_TARGET_X86_64_LINUX_RUSTFLAGS = ["-C" "link-arg=-fuse-ld=mold"];
  nativeBuildInputs = [
    autoPatchelfHook
    blueprint-compiler
    cargo
    desktop-file-utils
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
    libadwaita
    libjack2
    pango
  ];
})

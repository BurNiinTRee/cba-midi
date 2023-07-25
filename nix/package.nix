{
  alsaLib,
  autoPatchelfHook,
  cairo,
  copyDesktopItems,
  gdk-pixbuf,
  glib,
  graphene,
  gtk4,
  harfbuzz,
  makeDesktopItem,
  mold,
  pango,
  pkg-config,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  name = "cba-midi";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = [
    autoPatchelfHook
    copyDesktopItems
    mold
    pkg-config
  ];
  PREFIX = placeholder "out";
  desktopItems = [
    (
      makeDesktopItem {
        name = "cba-midi";
        desktopName = "CBA Keyboard";
        exec = "cba-midi";
      }
    )
  ];
  buildInputs = [
    alsaLib
    cairo
    gdk-pixbuf
    glib
    graphene
    gtk4
    harfbuzz
    pango
  ];
  postInstall = ''
    install -D -t $out/share/cba-midi share/cba-midi/map.txt
  '';
}

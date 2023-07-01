{
  inputs = {
    bntr.url = "github:BurNiinTRee/nix-sources";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {
    flake-parts,
    fenix,
    bntr,
    devenv,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} ({...}: {
      systems = ["x86_64-linux"];

      imports = [bntr.flakeModules.nixpkgs devenv.flakeModule];

      perSystem = {pkgs, ...}: {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "cba-midi";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = with pkgs; [
            autoPatchelfHook
            mold
            pkg-config
          ];
          PREFIX = placeholder "out";
          buildType = "debug";
          buildInputs = with pkgs; [
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
        };
        nixpkgs.overlays = [fenix.overlays.default];
        devenv.shells.default = {pkgs, ...}: {
          languages.c.enable = true;
          packages = [
            pkgs.fenix.stable.toolchain
            pkgs.rust-analyzer
            pkgs.mold
            # MIDI
            pkgs.alsaLib
            # GTK4
            pkgs.cairo
            pkgs.gdk-pixbuf
            pkgs.glib
            pkgs.graphene
            pkgs.gtk4
            pkgs.harfbuzz
            pkgs.pango
          ];
        };
      };
    });
}

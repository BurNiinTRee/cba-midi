{
  inputs = {
    bntr.url = "github:BurNiinTRee/nix-sources?dir=inner";
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
    flake-parts.lib.mkFlake {inherit inputs;} ({lib, ...}: {
      systems = ["x86_64-linux"];

      imports = [
        bntr.modules.flake.nixpkgs
        devenv.flakeModule
      ];

      perSystem = {config, pkgs, ...}: {
        packages = {
          default = pkgs.callPackage ./nix/package.nix {};
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

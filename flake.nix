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
      ];

      perSystem = {
        config,
        pkgs,
        ...
      }: {
        packages = {
          default = pkgs.callPackage ./nix/package.nix {};
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [config.packages.default];
          packages = [
            pkgs.flatpak-builder
            pkgs.rust-analyzer
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
            cairo
            gdk-pixbuf
            glib
            graphene
            gtk4
            harfbuzz
            pango
            pipewire.jack
          ]);
        };
        nixpkgs.overlays = [fenix.overlays.default];
      };
    });
}

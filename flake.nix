{
  inputs = {
    bntr.url = "github:BurNiinTRee/nix-sources?dir=modules";
    devenv.url = "git+file:///home/user/projects/devenv";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {
    flake-parts,
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

      debug = true;

      perSystem = {
        config,
        pkgs,
        ...
      }: {
        packages = {
          default = pkgs.callPackage ./nix/package.nix {};
        };
        devenv.shells.default = let config'  = config; in {config, ...}: {
          inputsFrom = [config'.packages.default];
          packages = [
            pkgs.flatpak-builder
            pkgs.rust-analyzer
            pkgs.python3
          ];
          env.GI_TYPELIB_PATH = "${config.env.DEVENV_PROFILE}/lib/girepository-1.0";
          env.LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
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
      };
    });
}

# Root flake so `nix run github:zackmsa777-a11y/rustfetch` and
# `nix profile install github:zackmsa777-a11y/rustfetch` work.
# Crate on crates.io: rustftechh; installed binary / app: rustfetch.
{
  description = "rustfetch — blazingly fast system information fetch (crate: rustftechh)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustfetch = pkgs.callPackage ./packaging/nix/default.nix { src = self; };
      in
      {
        packages.default = rustfetch;
        packages.rustfetch = rustfetch;

        apps.default = {
          type = "app";
          program = "${rustfetch}/bin/rustfetch";
        };
        apps.rustfetch = {
          type = "app";
          program = "${rustfetch}/bin/rustfetch";
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    )
    // {
      overlays.default = final: _prev: {
        rustfetch = final.callPackage ./packaging/nix/default.nix { src = self; };
      };
    };
}

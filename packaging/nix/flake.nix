# Nix flake for rustfetch (crates.io package name: rustftechh).
# Usage (from this directory or with path to the monorepo root):
#   nix build .#rustfetch
#   nix run .#rustfetch -- --version
#
# The GitHub repo stays rustfetch; the crates.io crate is rustftechh;
# the installed command / app is always `rustfetch`.

{
  description = "rustfetch — blazingly fast system information fetch (crate: rustftechh)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustfetch = pkgs.callPackage ./default.nix {
          src = ../..; # repository root (Cargo.toml lives two levels up)
        };
      in {
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
          packages = [ pkgs.cargo pkgs.rustc pkgs.rustfmt pkgs.clippy ];
        };
      });
}

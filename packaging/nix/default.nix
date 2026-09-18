# Nix expression building rustfetch from a source tree (repo root / flake `self`).
# Crate name on crates.io: rustftechh; binary: rustfetch.
#
# Prefer the root flake (`nix build .#rustfetch`). For a future nixpkgs PR see
# `nixpkgs.nix` (fetchCrate) and copy to `pkgs/by-name/ru/rustfetch/package.nix`.
{
  lib,
  rustPlatform,
  src ? ../..,
}:

rustPlatform.buildRustPackage rec {
  pname = "rustfetch";
  version = "0.1.0";

  inherit src;

  # Prefer vendoring from the committed Cargo.lock at the repository root.
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  # Binary is declared as [[bin]] name = "rustfetch" in Cargo.toml
  # (package name is rustftechh).
  meta = with lib; {
    description = "Blazingly fast system information fetch tool written in Rust";
    homepage = "https://github.com/zackmsa777-a11y/rustfetch";
    changelog = "https://github.com/zackmsa777-a11y/rustfetch/blob/master/CHANGELOG.md";
    license = with licenses; [
      mit
      asl20
    ];
    mainProgram = "rustfetch";
    platforms = platforms.unix ++ platforms.windows;
  };
}

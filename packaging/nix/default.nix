# Nix expression building rustfetch from the repository root.
# Crate name on crates.io: rustftechh; binary: rustfetch.
{
  lib,
  rustPlatform,
  src ? ../..,
}:

rustPlatform.buildRustPackage rec {
  pname = "rustfetch";
  version = "0.1.0";

  inherit src;

  # Keep in sync with Cargo.lock at the repo root after dependency bumps.
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  # Binary is declared as [[bin]] name = "rustfetch" in Cargo.toml
  # (package name is rustftechh).
  meta = with lib; {
    description = "Blazingly fast system information fetch tool written in Rust";
    homepage = "https://github.com/zackmsa777-a11y/rustfetch";
    license = with licenses; [ mit asl20 ];
    mainProgram = "rustfetch";
    platforms = platforms.unix ++ platforms.windows;
  };
}

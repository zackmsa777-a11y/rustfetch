# Candidate expression for a nixpkgs PR.
# Copy this file to: pkgs/by-name/ru/rustfetch/package.nix
#
# Naming:
#   - nixpkgs attribute / directory: rustfetch
#   - crates.io crate (fetchCrate pname): rustftechh
#   - installed binary (mainProgram): rustfetch
#
# Refresh hashes after a crates.io publish:
#   1. Set hash / cargoHash to lib.fakeHash (or delete them).
#   2. nix-build -E 'with import <nixpkgs> {}; callPackage ./packaging/nix/nixpkgs.nix {}'
#   3. Paste the "got:" SHA256 values from the build error into hash and cargoHash.
#   Or: nix-prefetch-url --type sha256 https://static.crates.io/crates/rustftechh/rustftechh-VERSION.crate
#
{
  lib,
  rustPlatform,
  fetchCrate,
}:

rustPlatform.buildRustPackage rec {
  pname = "rustfetch";
  version = "0.1.0";

  src = fetchCrate {
    pname = "rustftechh";
    inherit version;
    hash = "sha256-9gnfvRGw0FHnn8L2Q6vM6wMAYec6xcrR+MzcqI0ZAuw=";
  };

  cargoHash = "sha256-HI0RhTn0GvjVq/pof2ZICOnwJgtYr0SKac+i55M2ptc=";

  meta = {
    description = "Blazingly fast system information fetch tool written in Rust";
    homepage = "https://github.com/zackmsa777-a11y/rustfetch";
    changelog = "https://github.com/zackmsa777-a11y/rustfetch/blob/master/CHANGELOG.md";
    license = with lib.licenses; [
      mit
      asl20
    ];
    mainProgram = "rustfetch";
    maintainers = with lib.maintainers; [ ];
  };
}

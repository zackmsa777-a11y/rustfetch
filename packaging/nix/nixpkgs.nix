# Candidate expression for a nixpkgs PR / version bump.
# Copy to: pkgs/by-name/ru/rustftechh/package.nix (or rustfetch if free)
#
# After publishing a new crates.io version, refresh hashes:
#   nix-build -E 'with import <nixpkgs> {}; callPackage ./packaging/nix/nixpkgs.nix {}'
# and paste the got: sha256 values.
{
  lib,
  rustPlatform,
  fetchCrate,
}:

rustPlatform.buildRustPackage rec {
  pname = "rustftechh";
  version = "0.1.1";

  src = fetchCrate {
    pname = "rustftechh";
    inherit version;
    hash = lib.fakeHash;
  };

  cargoHash = lib.fakeHash;

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

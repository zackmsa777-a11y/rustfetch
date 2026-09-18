# Nix packaging

| File | Role |
| :--- | :--- |
| `../../flake.nix` | **Root flake** — `nix run github:zackmsa777-a11y/rustfetch`, `nix build .#rustfetch` |
| `default.nix` | `buildRustPackage` from repo source (`cargoLock.lockFile` → `Cargo.lock`) |
| `flake.nix` | Convenience flake when `cd packaging/nix` |
| `nixpkgs.nix` | Future nixpkgs PR (`fetchCrate` pname `rustftechh`) → copy to `pkgs/by-name/ru/rustfetch/package.nix` |

## Flake (recommended)

```bash
nix run github:zackmsa777-a11y/rustfetch -- --version
nix profile install github:zackmsa777-a11y/rustfetch
# from a checkout:
nix build .#rustfetch
./result/bin/rustfetch --version
```

## nixpkgs PR

1. Ensure version `0.1.0` (or newer) is on [crates.io/crates/rustftechh](https://crates.io/crates/rustftechh).
2. Update `version`, `hash`, and `cargoHash` in `nixpkgs.nix` (see comments there).
3. Copy to nixpkgs as `pkgs/by-name/ru/rustfetch/package.nix`.
4. Users (once merged): `nix-shell -p rustfetch` / `environment.systemPackages = [ pkgs.rustfetch ];`

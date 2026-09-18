# Packaging stubs

Templates for third-party package managers. **None of these are published** until a
tagged GitHub release exists and checksums are filled in.

## Naming

| Role | Name |
| :--- | :--- |
| GitHub repository | `zackmsa777-a11y/rustfetch` |
| crates.io package | `rustftechh` |
| Installed command / binary | `rustfetch` |
| docs.rs | `https://docs.rs/rustftechh` |

Release artifacts from `.github/workflows/release.yml` keep the `rustfetch-*` prefix
(binary and archive names). Install with crates.io via:

```bash
cargo install rustftechh --locked   # installs ~/.cargo/bin/rustfetch
```

## Files

| Path | Target |
| :--- | :--- |
| `homebrew/rustftechh.rb` | Homebrew formula (`class Rustftechh`; installs `rustfetch`) |
| `aur/PKGBUILD` + `aur/.SRCINFO` | Arch User Repository (`pkgname=rustftechh`, binary `/usr/bin/rustfetch`) |
| `scoop/rustftechh.json` | Scoop bucket manifest (`bin`: `rustfetch.exe`) |
| `nix/flake.nix` + `nix/default.nix` | Nix flake / package (`apps.rustfetch`) |

## Maintainer submit steps

### Homebrew tap

1. Tag `vX.Y.Z` and wait for release assets (or use the GitHub source archive).
2. Download the preferred tarball; set `url` / `sha256` / `version` in `homebrew/rustftechh.rb`.
3. Create or update a tap, e.g. `homebrew-rustfetch`, copy the formula as `rustftechh.rb`.
4. `brew audit --new --formula rustftechh` then open a PR / push the tap.
5. Users: `brew install <your-tap>/rustftechh` → command `rustfetch`.

### AUR

1. Fill `pkgver` / `sha256sums` in `aur/PKGBUILD` after the GitHub tag exists.
2. `makepkg --printsrcinfo > .SRCINFO` (see `aur/README.md`).
3. `git clone ssh://aur@aur.archlinux.org/rustftechh.git`, copy files, commit, push.
4. Users: `yay -S rustftechh` → `/usr/bin/rustfetch`.

### Scoop

1. After a Windows release zip exists, fill `hash` in `scoop/rustftechh.json`.
2. Add the manifest to a Scoop bucket (personal or community).
3. Users: `scoop install rustftechh` → `rustfetch.exe`.

### Nix

```bash
cd packaging/nix
nix build .#rustfetch
nix run .#rustfetch -- --version
```

Or call `default.nix` from a NixOS module / overlay with `src` pointing at the repo root.

Release binaries are produced by `.github/workflows/release.yml` on tags matching `v*`.

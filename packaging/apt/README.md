# Debian / Ubuntu / Mint `.deb` packaging

This repository ships **source packaging** under `/debian` and a helper script
that produces an installable `.deb` for Linux Mint, Ubuntu, and Debian.

| Role | Name |
| :--- | :--- |
| `.deb` / apt package | `rustftechh` |
| crates.io | `rustftechh` |
| Installed binary | `/usr/bin/rustfetch` |

## Status

**Not yet in the Debian or Ubuntu archives.** Users install `.deb` files from
[GitHub Releases](https://github.com/zackmsa777-a11y/rustfetch/releases) (or
build locally). There is no official `apt` repository yet.

## Install a release `.deb` (Mint / Ubuntu / Debian)

```bash
# Download rustftechh_*.deb from GitHub Releases, then:
sudo apt install ./rustftechh_*.deb
rustfetch --version
```

Or with `dpkg`:

```bash
sudo dpkg -i rustftechh_*.deb
sudo apt-get install -f   # only if dependencies need fixing
```

## Build a `.deb` locally

Requires `cargo` (rustup recommended; project needs Rust **1.88+**), `dpkg-deb`,
and usual build tools:

```bash
./packaging/apt/build-deb.sh
# -> packaging/apt/out/rustftechh_0.1.0-1_<arch>.deb
dpkg-deb -c packaging/apt/out/rustftechh_*.deb | grep usr/bin/rustfetch
sudo apt install ./packaging/apt/out/rustftechh_*.deb
```

The script builds with `cargo build --release --locked`, then assembles the
package tree (binary, man page, shell completions, licenses) and runs
`dpkg-deb --build`. This avoids depending on Debian’s older `cargo` package
while still matching the layout described by `debian/`.

Optional: full source package build (needs `debhelper`, and a new enough
`cargo`/`rustc` on `PATH`):

```bash
sudo apt-get install -y debhelper build-essential
dpkg-buildpackage -us -uc -b
```

## CI / GitHub Actions

Workflow template: [`deb.yml.template`](deb.yml.template).

- Builds a `.deb` on pull requests (artifact upload) and on tags `v*` (also
  attached to the GitHub Release when the workflow is active).
- If pushing `.github/workflows/*` fails due to a missing `workflow` OAuth
  scope, copy the template manually:

```bash
cp packaging/apt/deb.yml.template .github/workflows/deb.yml
git add .github/workflows/deb.yml && git commit -m "ci: build rustftechh .deb"
```

## Path to Debian / Ubuntu archives (later)

1. **ITP (Intent to Package)** — file a Debian WNPP bug:
   https://www.debian.org/devel/wnpp/ — subject like
   `ITP: rustftechh -- blazingly fast system information fetch tool`.
2. Polish `debian/` against Debian Policy and the [Rust team packaging
   guide](https://rust-team.pages.debian.net/book/) (dh-cargo / cargo vendor
   for official uploads).
3. Upload a source package to **[Debian Mentors](https://mentors.debian.net/)**
   and request sponsorship from the Debian Rust team or a general sponsor.
4. Once in Debian unstable, Ubuntu usually syncs automatically; for a faster
   Ubuntu-only path see a PPA on Launchpad.

Until then, this repo’s release `.deb` is the supported apt-style install path.

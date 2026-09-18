# AUR packaging notes

These files are **stubs** for maintainers. They are not uploaded to the AUR automatically.

**Naming:** AUR/`pkgname` is **`rustftechh`** (matches crates.io). The GitHub repo and
installed binary remain **`rustfetch`**. The package `provides`/`conflicts` `rustfetch`.

## Checklist before first AUR publish

1. Tag and publish a GitHub release (`vX.Y.Z`) so the source archive exists:
   `https://github.com/zackmsa777-a11y/rustfetch/archive/refs/tags/vX.Y.Z.tar.gz`
2. Compute checksums:
   ```bash
   curl -sL -o rustfetch-X.Y.Z.tar.gz \
     https://github.com/zackmsa777-a11y/rustfetch/archive/refs/tags/vX.Y.Z.tar.gz
   sha256sum rustfetch-X.Y.Z.tar.gz
   ```
3. Set `pkgver` / `sha256sums` in `PKGBUILD`.
4. Generate `.SRCINFO` (do **not** hand-edit long-term):
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
5. Clone the AUR repo and push:
   ```bash
   git clone ssh://aur@aur.archlinux.org/rustftechh.git
   # copy PKGBUILD + .SRCINFO, commit, push
   ```

A committed `.SRCINFO` template with placeholders lives beside this README for reference only;
regenerate it with `makepkg --printsrcinfo` before any real upload.

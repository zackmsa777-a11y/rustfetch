# Homebrew formula template for rustftechh (binary: rustfetch).
#
# GitHub repo stays zackmsa777-a11y/rustfetch; crates.io / formula name is rustftechh.
#
# After cutting a GitHub release (tag vX.Y.Z) and uploading archives:
#   1. Prefer a release tarball URL (binary bottle) when available, e.g.:
#        https://github.com/zackmsa777-a11y/rustfetch/releases/download/vX.Y.Z/rustfetch-vX.Y.Z-aarch64-apple-darwin.tar.gz
#      Or build from source via the GitHub archive:
#        https://github.com/zackmsa777-a11y/rustfetch/archive/refs/tags/vX.Y.Z.tar.gz
#   2. Fill `sha256` with: shasum -a 256 <downloaded-file>
#   3. Bump `version` to match the tag (without leading "v").
#   4. Submit to a tap or homebrew-core:
#        brew audit --new --formula rustftechh
#
# This file is a stub checked into the repo for maintainers — not published until
# someone submits it after a real release.

class Rustftechh < Formula
  desc "Blazingly fast system information fetch tool written in Rust"
  homepage "https://github.com/zackmsa777-a11y/rustfetch"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/zackmsa777-a11y/rustfetch.git", branch: "master"

  # REPLACE after release — preferred: GitHub release source archive
  url "https://github.com/zackmsa777-a11y/rustfetch/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "REPLACE_WITH_SHA256_OF_SOURCE_TARBALL"
  version "0.1.1"

  # Only needed when building from source (url above is a source archive).
  # Drop this dependency if you switch `url` to a prebuilt release binary tarball
  # and install the `rustfetch` binary directly in `install`.
  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    # cargo install places the [[bin]] named rustfetch into bin/
    man1.install "man/rustfetch.1" if File.exist?("man/rustfetch.1")
    bash_completion.install "completions/rustfetch.bash" => "rustfetch" if File.exist?("completions/rustfetch.bash")
    zsh_completion.install "completions/rustfetch.zsh" => "_rustfetch" if File.exist?("completions/rustfetch.zsh")
    fish_completion.install "completions/rustfetch.fish" if File.exist?("completions/rustfetch.fish")
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/rustfetch --version")
  end
end

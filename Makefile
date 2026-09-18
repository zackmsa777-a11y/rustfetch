# rustfetch developer helpers
# Usage: make install | make completions-install | make man-install

PREFIX       ?= $(HOME)/.local
BINDIR       ?= $(PREFIX)/bin
MANDIR       ?= $(PREFIX)/share/man/man1
CARGO_BIN    ?= $(HOME)/.cargo/bin
BASH_COMPDIR ?= $(PREFIX)/share/bash-completion/completions
ZSH_COMPDIR  ?= $(PREFIX)/share/zsh/site-functions
FISH_COMPDIR ?= $(PREFIX)/share/fish/vendor_completions.d

.PHONY: all build release test install uninstall completions-install man-install publish-dry-run clean help

all: release

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

# Install the release binary to BINDIR (default: ~/.local/bin).
# Falls back to cargo install --path when preferred.
install: release
	mkdir -p "$(BINDIR)"
	install -m755 target/release/rustfetch "$(BINDIR)/rustfetch"
	@echo "Installed $(BINDIR)/rustfetch"
	@echo "Tip: also run 'make completions-install' and 'make man-install'"

# Alternative: cargo-managed install into ~/.cargo/bin
install-cargo:
	cargo install --path . --force --locked

uninstall:
	rm -f "$(BINDIR)/rustfetch"
	rm -f "$(CARGO_BIN)/rustfetch"
	@echo "Removed rustfetch from $(BINDIR) (and $(CARGO_BIN) if present)"

completions-install: release
	mkdir -p "$(BASH_COMPDIR)" "$(ZSH_COMPDIR)" "$(FISH_COMPDIR)"
	./target/release/rustfetch --completions bash > "$(BASH_COMPDIR)/rustfetch"
	./target/release/rustfetch --completions zsh > "$(ZSH_COMPDIR)/_rustfetch"
	./target/release/rustfetch --completions fish > "$(FISH_COMPDIR)/rustfetch.fish"
	@echo "Completions installed under $(PREFIX)/share/..."

man-install:
	mkdir -p "$(MANDIR)"
	install -m644 man/rustfetch.1 "$(MANDIR)/rustfetch.1"
	@echo "Man page installed to $(MANDIR)/rustfetch.1"
	@mandb "$(PREFIX)/share/man" 2>/dev/null || true

# Never uploads to crates.io — dry-run only.
publish-dry-run:
	cargo publish --dry-run

clean:
	cargo clean

help:
	@echo "Targets: build release test install install-cargo uninstall"
	@echo "         completions-install man-install publish-dry-run clean"
	@echo "PREFIX=$(PREFIX) BINDIR=$(BINDIR)"
	@echo "crates.io: cargo install rustftechh --locked  (binary: rustfetch)"

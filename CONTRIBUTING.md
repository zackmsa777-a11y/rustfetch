# Contributing to rustfetch

Guidelines and standards for contributing to rustfetch.

---

## Development Setup

### Prerequisites

- Rust toolchain (version 1.70.0 or later, stable recommended)
- Cargo, clippy, and rustfmt

```bash
rustup update stable
rustup component add clippy rustfmt
```

### Cloning & Building

```bash
git clone https://github.com/zackmsa777-a11y/rustfetch.git
cd rustfetch
cargo build
cargo run
```

---

## Coding Standards

1. **Zero Comment Rule in Source Files**:
   Source files under `src/` must contain zero code comments. Code must be self-documenting with clear, expressive naming, small helper functions, and explicit type signatures.
2. **Formatting**:
   Run `cargo fmt -- --check` before committing.
3. **Lints**:
   Ensure zero clippy warnings: `cargo clippy -- -D warnings`.
4. **Testing**:
   Every new module or parser must have unit test coverage under `mod tests` in `src/main.rs` or corresponding modules. Run `cargo test`.
5. **Blazing Performance**:
   Probes must avoid spawning subshells or heavy commands when direct procfs, sysfs, or libc queries are feasible. Concurrent queries should use scoped threads (`std::thread::scope`).

---

## Adding a New Distro or OS Logo

1. Open `src/logos.rs`.
2. Add your ASCII art array as a `pub const <DISTRO>_LOGO: &[&str] = &[ ... ];`.
3. Add the identifier to `ALL_LOGOS`.
4. Add the pattern match branch in `get_logo()` with the distro's authentic ANSI color escape code.
5. Add a unit test verifying logo retrieval in `src/main.rs`.

---

## Adding a New Information Module

1. Define the module probe in `src/info/<module>.rs`.
2. Add the field to `SystemInfo` in `src/info/types.rs`.
3. Call the detector in `src/info/mod.rs` (concurrently spawned in `thread::scope` if I/O bound).
4. Register the display format in `src/printer.rs`.
5. Document the module in `src/cli.rs` (`print_modules()`) and `README.md`.

---

## Submitting a Pull Request

1. Fork the repository on GitHub.
2. Create a feature branch: `git checkout -b feature/my-new-logo`.
3. Commit your changes: `git commit -m "feat(logo): add gentoo logo"`.
4. Push to your fork: `git push origin feature/my-new-logo`.
5. Open a Pull Request on GitHub with a description of your changes.

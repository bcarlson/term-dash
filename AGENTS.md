# Repository Guidelines

## Project Structure & Module Organization
Follow standard crate layout: keep `Cargo.toml` at the root and put the runtime entry in `src/main.rs` (or `src/bin/term-dash.rs` once multiple binaries exist). Organize reusable logic under `src/` modules such as `src/widgets/` for TUI components, `src/data/` for adapters, and `src/app.rs` for orchestration. Store sample configs or ASCII templates under `assets/` or `examples/`. Co-locate focused unit tests with their modules using `#[cfg(test)]`, and keep integration flows in `tests/`.

## Build, Test, and Development Commands
- `cargo build`: compile the crate and surface borrow-checker issues early.
- `cargo run --bin term-dash`: run the dashboard binary (adjust if more bins appear).
- `cargo test`: execute unit and integration tests.
- `cargo fmt && cargo clippy --all-targets --all-features`: enforce formatting and linting in one step.

## Coding Style & Naming Conventions
Run `rustfmt` before committing (4-space indentation, 100-column wrapping). Follow idiomatic Rust naming: snake_case for files/modules/functions, CamelCase for types/traits, SCREAMING_SNAKE_CASE for constants. Keep modules cohesive, prefer pure functions returning `Result<T, anyhow::Error>`, and isolate side effects near the entry point. Document public APIs with Rustdoc and add inline notes only when rendering logic is non-obvious.

## Testing Guidelines
Each new module should include `#[cfg(test)]` unit coverage, while behavior-level scenarios go into `tests/` with filenames such as `tests/layout_resize.rs`. Ensure both `cargo test` and `cargo test --release` pass before opening a PR.

## Commit & Pull Request Guidelines
Use concise, imperative commit messages: `feat(layout): add grid widget` or `fix(data): guard empty payload`. Group related work so each commit builds cleanly. Pull requests should summarize the change, list verification steps, link issues, and include terminal screenshots or recordings when UI output shifts.

## Code Review & Approvals
Create topic branches from `main` (e.g., `git checkout -b feature/layout-grid`) and push them for review against `protentions`. All merges into `main` require approval from Brian Carlson; request their review explicitly and block merging until they approve. Highlight risky areas, attach logs or recordings for UI updates, and note follow-up issues so the approver can sign off confidently.

## Local Environment Setup
Install the latest stable toolchain via `rustup` and add the `clippy` and `rustfmt` components. `cargo install cargo-watch cargo-udeps` keeps feedback fast and dependencies tidy; a typical loop is `cargo watch -x "check" -x "test"`. Capture screenshots at 24×80 or wider so comparisons stay consistent.

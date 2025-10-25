# Contributing to term-dash

Thanks for your interest in improving term-dash! This document outlines the contributor process and points you to deeper references.

## Code of Conduct
Participation is governed by the [Code of Conduct](CODE_OF_CONDUCT.md). Please make sure your interactions stay respectful and inclusive.

## Getting Started
1. Review the repository playbook in [AGENTS.md](AGENTS.md) for expectations on project layout, coding style, testing, and approvals.
2. Fork the repository or create a topic branch from `main` (or `protentions` while the bootstrap branch is active).
3. Install the latest Rust toolchain with `rustup`, plus `clippy` and `rustfmt` components.

## Development Workflow
- Prefer small, focused commits with imperative messages (e.g., `feat(widget): add sparkline renderer`).
- Keep code formatted via `cargo fmt` and linted with `cargo clippy --all-targets --all-features`.
- Run `cargo test` (and `cargo test --release` for performance-sensitive changes) before submitting.
- Update docs, examples, and fixtures when behavior changes.

## Pull Requests
- Draft PRs when work is in progress; convert to ready-for-review once checks pass.
- Fill out the pull request template, including verification steps and screenshots for UI changes.
- Brian Carlson is currently the required approver for merges to `main`; request their review explicitly.

## Reporting Issues
Use the provided issue templates for bugs, feature ideas, and maintenance tasks. Include reproduction steps, environment details, and proposed solutions when possible.

## Questions
Open a GitHub Discussion (once enabled) or file an issue using the “Task” template to ask for clarification. You can also mention `@bcarlson` directly on GitHub for guidance.

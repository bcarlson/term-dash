# term-dash

term-dash is an upcoming Rust-powered terminal dashboard for monitoring systems and developer workflows from the command line. The project is still in its bootstrap phase; use this README as the canonical source for roadmap, setup, and contribution guidance.

## Project Status
Active development has not yet begun. Initial tasks include seeding the Rust crate structure, defining core widgets, and wiring data adapters. Track progress via GitHub Projects or the issue tracker.

## Getting Started
1. Install the latest stable Rust toolchain with `rustup`.
2. Clone the repository and create a feature branch from `protentions` until `main` becomes the default development branch.
3. Run `cargo build` once the crate lands to verify your environment.

## Development Workflow
- Follow the repository playbook in `AGENTS.md` for module layout, coding standards, and review expectations.
- Use `cargo fmt`, `cargo clippy --all-targets --all-features`, and `cargo test` before opening a pull request.
- Capture terminal screenshots at 24×80 or wider when UI output changes.

## Contributing
We welcome issues and pull requests for features, bug fixes, documentation, and tooling. Please:
- Review `CONTRIBUTING.md` for contribution logistics.
- Read the Code of Conduct to ensure a welcoming environment.
- Use the issue templates provided for bugs, feature requests, and tasks.

## Security
Report vulnerabilities privately via the Security advisory form linked in `SECURITY.md`. Please do not open public issues for security concerns.

## License
term-dash is released under the MIT License. See `LICENSE` for details.

# term-dash

term-dash is an upcoming Rust-powered terminal dashboard for monitoring systems and developer workflows from the command line. The project is still in its bootstrap phase; use this README as the canonical source for roadmap, setup, and contribution guidance.

## Project Status
Active development has not yet begun. Initial tasks include seeding the Rust crate structure, defining core widgets, and wiring data adapters. Track progress via GitHub Projects or the issue tracker.

## Getting Started
1. Install the latest stable Rust toolchain with `rustup`.
2. Clone the repository and create a feature branch from `protentions` until `main` becomes the default development branch.
3. Run `cargo run` to launch the retro clock demo (press `q` to quit).

## Development Workflow
- Follow the repository playbook in `AGENTS.md` for module layout, coding standards, and review expectations.
- Use `cargo fmt`, `cargo clippy --all-targets --all-features`, and `cargo test` before opening a pull request.
- Capture terminal screenshots at 24×80 or wider when UI output changes.

## Retro Clock Demo
- Rendered with ASCII pipes, segments, and dots to mimic an 80s digital clock.
- Resizes automatically to fill the available terminal space; window changes apply roughly every 200ms.
- Falls back to a compact readout if the window becomes too small.

## Docker Workflow
1. Install Docker Desktop (or Engine) and Docker Compose.
2. Build the image with `docker compose build`.
3. Run commands inside the container, for example:
   - `docker compose run --rm dev cargo fmt`
   - `docker compose run --rm dev cargo test`
   - `docker compose run --rm dev cargo run`
4. The container mounts the repository at `/workspace`; results sync back to your host.
5. Override the user and group IDs by exporting `UID`/`GID` before invoking Compose to align container permissions with your local user.

## Contributing
We welcome issues and pull requests for features, bug fixes, documentation, and tooling. Please:
- Review `CONTRIBUTING.md` for contribution logistics.
- Read the Code of Conduct to ensure a welcoming environment.
- Use the issue templates provided for bugs, feature requests, and tasks.

## Security
Report vulnerabilities privately via the Security advisory form linked in `SECURITY.md`. Please do not open public issues for security concerns.

## License
term-dash is released under the MIT License. See `LICENSE` for details.

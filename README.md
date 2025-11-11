# term-dash

term-dash is a Rust-powered terminal dashboard for monitoring systems and developer workflows from the command line. The current prototype ships a dual-screen experience that mixes a retro clock with a commute-oriented route board, plus the plumbing needed for future widgets.

## Project Status
The application already includes a working event loop, multiple rendering paths, configuration persistence, and a location-aware route view. The sections below summarize the features that are live today so contributors and testers have an authoritative reference.

## Features

### Dashboard Core
- Alternate-screen interface built with crossterm; refreshes roughly every 200 ms and automatically reflows to the current terminal size.
- Left/Right arrows toggle between the clock screen and the route board, giving you two widgets inside one binary.
- Graceful degradation for tiny windows: the clock falls back to a compact readout and the route board displays a resize warning until enough space is available.

### Clock Screens
- Two clock faces (`Retro` and `Modern`) that you can cycle with ↑/↓ while in clock mode; both render the current time in 12-hour format.
- Retro face draws seven-segment “LED” digits with ASCII fallbacks, layered highlights, and an auto-detected color palette that respects `NO_COLOR` and can be overridden via `TERM_DASH_FORCE_MODE=ascii|led|light|dark`.
- Modern face uses scalable glyph patterns to fill large terminals, adds AM/PM plus weekday and date metadata, and automatically switches to a compact column layout when space is limited.
- Rendering is palette-aware: when a terminal background is detected as dark/light (via `$COLORFGBG`) the colors adjust for legibility.

### Route Board
- Shows the current network-derived location (`Home`, `Work`, or the SSID name) and highlights the opposite destination to hint at the likely next commute leg.
- Displays synthetic ETAs for saved home/work addresses, including departure countdowns, arrival times, traffic severity (“normal” vs “heavy”), and whether an “accident” was detected for that snapshot.
- Detects when an address is missing and prints a call-to-action that tells you which key to press to configure it.
- Provides inline prompts for entering or updating addresses (press `H`/`W` while on the route screen); the existing value is pre-populated, the text cursor is visible, and status messages appear when a save succeeds or fails.
- Offers a footer with contextual hints: keyboard shortcuts when idle, the active text prompt while editing, or status/error messages after operations.

### Configuration & Network Awareness
- Addresses persist to `~/.termdash/config` as pretty-printed JSON through the `AppConfig` module, so the commute board survives restarts.
- Network SSIDs are detected via `networksetup` on macOS and `iwgetid` on Linux; you can override detection with `TERMDASH_FORCE_SSID=<value>` or specify a Wi-Fi device list via `TERMDASH_WIFI_DEVICE=en0,en1`.
- SSID heuristics (`nevis` = Home, `cubic` = Work) inform the highlight state, but an `Unknown` SSID is still shown so you know which network was observed.
- Time-of-day, sine-wave jitter, and periodic “accident” events feed the travel-time simulation, creating believable variability without contacting external APIs.

## Getting Started
1. Install the latest stable Rust toolchain with `rustup`.
2. Clone the repository and create a feature branch from `main` (open PRs against `protentions` per `AGENTS.md`).
3. Run `cargo run` (or `cargo run --bin term-dash`) to launch the dashboard; use `→` to switch to the route board and press `q`/`Esc` when you’re done.

## Keyboard Shortcuts
- `←` / `→`: Switch between the clock screen and the route board.
- `↑` / `↓`: Cycle through available clock faces (only when the clock screen is focused).
- `H` / `W`: Start editing the Home or Work address while viewing the route board; existing values seed the prompt.
- `Enter`: Save the in-progress address to `~/.termdash/config`.
- `Esc`: Cancel an address prompt or quit the app when no prompt is active (you can also press `q` to quit).

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

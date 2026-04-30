```
    /\
   /__\    PhantomDev
  /    \   Humanizer for AI Agents & Commits
 /______\  github.com/John-Varghese-EH/PhantomDev
```

<div align="center">

  <img src="assets/logo.svg" alt="PhantomDev Logo" width="100" />

# PhantomDev

**Adversarial Stylometry Framework for the AI-Augmented Developer**

> Inject human entropy back into your workflow.

[![Crates.io](https://img.shields.io/crates/v/phantomdev.svg)](https://crates.io/crates/phantomdev)
[![Docs.rs](https://docs.rs/phantomdev/badge.svg)](https://docs.rs/phantomdev)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE)
[![Build](https://github.com/John-Varghese-EH/PhantomDev/actions/workflows/ci.yml/badge.svg)](https://github.com/John-Varghese-EH/PhantomDev/actions)
[![GitHub Pages](https://img.shields.io/badge/docs-github--pages-blue.svg)](https://john-varghese-eh.github.io/PhantomDev/)

</div>

---

## Overview

In 2026, AI-generated code carries an identifiable signature. Whether it's academic integrity systems, hiring pipelines, or code review tools, pattern-based detection is increasingly used to flag work that doesn't "look human."

**PhantomDev** addresses this directly. It's not about deception-it's about **stylometric sovereignty**. Developers should be able to use high-productivity AI tooling without sacrificing ownership of their own coding identity.

PhantomDev learns your personal style from your repository history and applies it back to AI-assisted output-transforming variable names, comment phrasing, line cadence, and commit message structure to match how *you* actually write code.

---

## Features

### AI Detection

- Local inference using RoBERTa-based models (no API required)
- Cloud fallback via OpenRouter and Anthropic
- Pattern detection for common AI tells: watermarks, uniform commenting, emoji overuse
- Multi-language support: Rust, Python, JavaScript/TypeScript, Go, C++

### Code Humanization

- Repository-aware style learning from your commit history
- Variable and function renaming to match personal conventions
- Targeted comment injection and removal
- Entropy injection for natural stylistic variation

### Temporal Obfuscation (Jitter Engine)

- Delayed staging buffer to simulate organic development pace
- Configurable timing windows (min/max delay in seconds)
- Designed to avoid commit-frequency fingerprinting

### Stealth Scoring

- Real-time per-file stealth score
- Pattern-based breakdown of detected AI signatures
- Threshold-configurable alerting

### TUI Dashboard

- Terminal UI for live stealth score monitoring
- Detection heatmap per file
- Style profile visualization

### Undercover Mode

- Commit message transformation via adversarial stylometry
- Code comment rewriting with human-idiomatic phrasing
- Variable name substitution based on personal naming patterns

### Pre-Commit Integration

- Git hook-based detection before every commit
- Auto-fix mode for common AI patterns
- Cross-platform, zero-dependency hook scripts

---

## Installation

### Recommended (Easy Install)

Clone the repository and run the built-in installer:

```bash
git clone https://github.com/John-Varghese-EH/PhantomDev.git
cd PhantomDev
cargo run -- easy-install
```

To also install IDE-specific skill files:

```bash
cargo run -- easy-install --ide claude
cargo run -- easy-install --ide cursor
cargo run -- easy-install --ide windsurf
cargo run -- easy-install --ide antigravity
```

### From Source

```bash
git clone https://github.com/John-Varghese-EH/PhantomDev.git
cd PhantomDev
cargo build --release
cargo install --path crates/cli
```

### One-Click Installer

```bash
curl -sSL https://john-varghese-eh.github.io/PhantomDev/install.sh | bash
```

---

## Quick Start

### Step 1: Install

```bash
curl -sSL https://john-varghese-eh.github.io/PhantomDev/install.sh | bash
```

### Step 2: Go to your project

```bash
cd your-project
```

### Step 3: Check your code

```bash
phantomdev
```

This will show your stealth score and suggest what to do next.

### Step 4: Fix AI patterns

```bash
phantomdev fix
```

That's it! Your code is now humanized.

---

## Usage

PhantomDev is designed to be simple. Just run `phantomdev` and it will guide you.

### Basic Commands

```bash
phantomdev              # Check status and get suggestions
phantomdev fix          # Auto-fix AI patterns in staged files
phantomdev scan         # See what's detected
phantomdev score         # Check your stealth score
phantomdev dashboard    # Launch visual dashboard
phantomdev config       # View or change settings
phantomdev install      # Install IDE integration
```

### Typical Workflow

1. **Stage your files**
   ```bash
   git add your-files
   ```

2. **Check what's detected**
   ```bash
   phantomdev scan
   ```

3. **Fix AI patterns**
   ```bash
   phantomdev fix
   ```

4. **Commit**
   ```bash
   git commit -m "your message"
   ```

### Advanced Options

```bash
# Fix specific files
phantomdev fix --files src/main.rs

# See what would change without applying
phantomdev fix --dry-run

# Detailed score breakdown
phantomdev score --detailed

# Reset settings to defaults
phantomdev config --reset

# Install IDE integration
phantomdev install --ide claude
phantomdev install --ide all
```
pre-commit install
pre-commit run --all-files
```

---

## Configuration

PhantomDev reads from `.phantomdev/config.toml`:

```toml
[detection]
threshold = 0.15              # AI probability threshold (0.0 - 1.0)
use_local = true              # Prioritize local models
use_cloud_fallback = true     # Fall back to cloud API if local score is uncertain

[humanization]
auto_humanize = false         # Run humanization automatically on commit
entropy_level = 0.5           # Transformation intensity (0.0 - 1.0)

[jitter]
enabled = false               # Enable temporal obfuscation
min_delay_secs = 60
max_delay_secs = 300

[api]
openrouter_key = ""           # Optional: OpenRouter API key
anthropic_key = ""            # Optional: Anthropic API key
base_url = ""                 # Optional: Custom API base URL
```

---

## Architecture

```
phantomdev/
├── crates/
│   ├── core/          # Shared types and traits
│   ├── detector/      # AI detection engine
│   ├── humanizer/     # Code transformation logic
│   ├── jitter/        # Temporal obfuscation engine
│   ├── undercover/    # Adversarial stylometry transformer
│   ├── cli/           # Main binary
│   └── tui/           # Terminal UI
├── rules/             # IDE rule files (Claude, Cursor, Windsurf, Antigravity)
├── skills/            # IDE skill files for agent frameworks
├── hooks/             # Git hook scripts
├── scripts/           # Pre-commit and utility scripts
└── models/            # Local model storage
```

---

## Testing

```bash
# All tests
cargo test

# With stdout output
cargo test -- --nocapture

# Single test by name
cargo test test_detector_creation

# Per crate
cargo test -p phantomdev-core
cargo test -p phantomdev-detector
cargo test -p phantomdev-humanizer
cargo test -p phantomdev-jitter
cargo test -p phantomdev-undercover
cargo test -p phantomdev-tui
```

## Building

```bash
cargo build                  # Debug
cargo build --release        # Optimized

# Cross-compile targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## Published on crates.io

PhantomDev is officially published on crates.io. You can easily install the CLI using cargo:

```bash
cargo install phantomdev
```

*(For detailed workspace publishing instructions, see [cargo-instruction.md](cargo-instruction.md))*

## Documentation

Full documentation is available at [john-varghese-eh.github.io/PhantomDev](https://john-varghese-eh.github.io/PhantomDev/)

---

## Security

PhantomDev is built with a local-first design:

- All detection runs locally by default; cloud APIs are strictly opt-in
- No telemetry or usage data is collected
- Fully open-source; all code is auditable
- Cloud fallback can be disabled entirely via configuration

See [SECURITY.md](SECURITY.md) for the responsible disclosure policy.

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a pull request. Follow the existing code style and include tests for new behavior.

## License

This project is licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Disclaimer

**For Educational and Experimental Purposes Only.** 
PhantomDev is provided "as is" to explore adversarial stylometry and help developers protect their coding signatures. It is not intended to bypass academic integrity systems, cheat in evaluations, or be used maliciously. The author (John Varghese) and contributors are not liable for any misuse of this tool or any consequences arising from its usage. Use responsibly.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- ML inference via [Candle](https://github.com/huggingface/candle) (HuggingFace)
- Terminal UI via [Ratatui](https://github.com/ratatui-org/ratatui)

## 💗 Support the Project

This is an active, evolving project. If it helped you, consider supporting continued development ☺️:

[![Buy me a Coffee](https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/CyberTrinity)
[![Patreon](https://img.shields.io/badge/Patreon-F96854?style=for-the-badge&logo=patreon&logoColor=white)](https://patreon.com/CyberTrinity)
[![Sponsor](https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white)](https://github.com/sponsors/John-Varghese-EH)

---

## Author

[John Varghese (J0X)](https://github.com/John-Varghese-EH)
- GitHub: [John-Varghese-EH](https://github.com/John-Varghese-EH)
- LinkedIn: [John Varghese](https://linkedin.com/in/John--Varghese)

---

<div align="center">

**⭐ Star this repo if it helped you.**

*PhantomDev - preserve your coding signature in the age of AI.*

</div>
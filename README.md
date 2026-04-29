    /\
   /__\    PhantomDev
  /    \   Humanizer for AI Agents & Commits
 /______\  github.com/John-Varghese-EH/PhantomDev

# PhantomDev

**The Adversarial Stylometry Framework for the AI-Augmented Developer**

> Inject human entropy back into your workflow.

## 🎯 Vision

In 2026, the "AI-generated" tag is becoming a new form of technical debt. Whether it's academic detectors flagging student code or recruiters scanning GitHub histories for "lazy" AI patterns, there is a growing need for **Stylistic Sovereignty**.

**PhantomDev** isn't about "cheating"; it's about **privacy and professional branding**. It allows developers to use high-productivity AI tools while ensuring the final output matches their own unique "human" signature—preserving the soul of the codebase.

## ✨ Features

### 🔍 AI Detection
- Local model support (RoBERTa-based detection)
- Cloud API fallback (OpenRouter, Anthropic)
- Pattern detection (watermarks, emoji overuse, uniform comments)
- Multi-language support (Rust, Python, JavaScript/TypeScript, Go, C++)

### 🎭 Code Humanization
- Repo-specific style learning
- Variable/function renaming
- Comment injection/removal
- Entropy injection for natural variation

### ⏱️ Temporal Obfuscation (Jitter Engine)
- Delayed staging buffer
- Simulated human development cycles
- Configurable timing patterns

### 📊 Stealth Scoring
- Real-time stealth score calculation
- Per-file analysis
- Pattern-based detection

### 🖥️ TUI Dashboard
- Visual stealth score display
- Detection heatmap
- Style profile visualization

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/John-Varghese-EH/PhantomDev.git
cd PhantomDev

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

### One-Click Installer (Coming Soon)

```bash
curl -sSL https://phantomdev.io/install.sh | bash
```

## 📖 Usage

### Initialize PhantomDev

```bash
phantomdev init
```

This creates a `.phantomdev` directory with configuration files.

### Scan for AI-Generated Content

```bash
# Scan staged files
phantomdev scan

# Scan specific files
phantomdev scan --files src/main.rs src/lib.rs

# Show detailed output
phantomdev scan --verbose
```

### Humanize Code

```bash
# Humanize staged files
phantomdev humanize

# Humanize specific files
phantomdev humanize --files src/main.rs

# Set entropy level
phantomdev humanize --entropy 0.7
```

### Check Stealth Score

```bash
# Show overall score
phantomdev score

# Show detailed breakdown
phantomdev score --detailed
```

### Launch Dashboard

```bash
phantomdev dashboard
```

### Configure

```bash
# Show current configuration
phantomdev config --show

# Reset to defaults
phantomdev config --reset
```

### Install Git Hooks

```bash
# Copy hooks to .git/hooks
cp hooks/pre-commit .git/hooks/
cp hooks/commit-msg .git/hooks/
chmod +x .git/hooks/pre-commit .git/hooks/commit-msg
```

## ⚙️ Configuration

PhantomDev uses a TOML configuration file located at `.phantomdev/config.toml`:

```toml
[detection]
threshold = 0.15              # AI probability threshold
use_local = true              # Use local models
use_cloud_fallback = true     # Use cloud API as fallback

[humanization]
auto_humanize = false         # Auto-humanize on commit
entropy_level = 0.5           # Entropy level (0.0 - 1.0)

[jitter]
enabled = false               # Enable temporal obfuscation
min_delay_secs = 60           # Minimum delay in seconds
max_delay_secs = 300          # Maximum delay in seconds

[api]
openrouter_key = ""           # OpenRouter API key (optional)
anthropic_key = ""            # Anthropic API key (optional)
base_url = ""                 # API base URL (optional)
```

## 🏗️ Architecture

```
phantomdev/
├── crates/
│   ├── core/          # Core library (types, traits)
│   ├── detector/      # AI detection engine
│   ├── humanizer/     # Code transformation
│   ├── jitter/        # Temporal obfuscation
│   ├── cli/           # Main CLI binary
│   └── tui/           # Terminal UI dashboard
├── rules/             # IDE rules (Cursor, Claude, etc.)
├── hooks/             # Git hooks
└── models/            # Local model storage
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_detector_creation

# Run tests for specific crate
cargo test -p phantomdev-core
cargo test -p phantomdev-detector
cargo test -p phantomdev-humanizer
cargo test -p phantomdev-jitter
cargo test -p phantomdev-tui
```

## 🏗️ Building

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## 📦 Publishing

```bash
# Publish to crates.io
cargo publish -p phantomdev-core
cargo publish -p phantomdev-detector
cargo publish -p phantomdev-humanizer
cargo publish -p phantomdev-jitter
cargo publish -p phantomdev-tui
cargo publish -p phantomdev
```

## 🔒 Security

PhantomDev is designed with security in mind:

- **Local-first detection**: Prioritizes local models over cloud APIs
- **No telemetry**: Does not collect usage data
- **Open source**: All code is available for audit
- **Configurable cloud fallback**: Can disable cloud API usage entirely

For security concerns, see [SECURITY.md](SECURITY.md).

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting PRs.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 📜 Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- ML inference powered by [Candle](https://github.com/huggingface/candle)
- TUI powered by [Ratatui](https://github.com/ratatui-org/ratatui)

## 📞 Support

- GitHub Issues: [https://github.com/John-Varghese-EH/PhantomDev/issues](https://github.com/John-Varghese-EH/PhantomDev/issues)
- Discord: [https://discord.gg/phantomdev](https://discord.gg/phantomdev)

---

**PhantomDev** - Preserve your coding signature in the age of AI.

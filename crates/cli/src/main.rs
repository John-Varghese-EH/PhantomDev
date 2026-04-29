//! PhantomDev CLI
//!
//! The main command-line interface for PhantomDev.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use phantomdev_core::{CodeBlock, Config, Detector, Humanizer, Language};
use phantomdev_detector::PhantomDetector;
use phantomdev_humanizer::PhantomHumanizer;
use phantomdev_jitter::PhantomJitter;
use phantomdev_tui::PhantomTui;
use std::path::PathBuf;

/// PhantomDev - The Adversarial Stylometry Framework for the AI-Augmented Developer
#[derive(Parser)]
#[command(name = "phantomdev")]
#[command(about = "Inject human entropy back into your workflow", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize PhantomDev in the current repository
    Init {
        /// Force reinitialization
        #[arg(short, long)]
        force: bool,
    },
    /// Scan current changes for AI-generated content
    Scan {
        /// Specific files to scan
        #[arg(short, long)]
        files: Vec<String>,
        /// Show detailed output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Humanize code to match repository style
    Humanize {
        /// Files to humanize
        #[arg(short, long)]
        files: Vec<String>,
        /// Entropy level (0.0 - 1.0)
        #[arg(short, long)]
        entropy: Option<f32>,
    },
    /// Show stealth score for current changes
    Score {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },
    /// Launch TUI dashboard
    Dashboard,
    /// Configure PhantomDev
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
        /// Reset to default configuration
        #[arg(short, long)]
        reset: bool,
    },
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => cmd_init(force),
        Commands::Scan { files, verbose } => cmd_scan(files, verbose),
        Commands::Humanize { files, entropy } => cmd_humanize(files, entropy),
        Commands::Score { detailed } => cmd_score(detailed),
        Commands::Dashboard => cmd_dashboard(),
        Commands::Config { show, reset } => cmd_config(show, reset),
    }
}

/// Initialize PhantomDev in the current repository
fn cmd_init(force: bool) -> Result<()> {
    println!("{}", "Initializing PhantomDev...".cyan());

    let config_path = PathBuf::from(".phantomdev/config.toml");
    if config_path.exists() && !force {
        println!("{}", "PhantomDev is already initialized. Use --force to reinitialize.".yellow());
        return Ok(());
    }

    // Create config directory
    std::fs::create_dir_all(".phantomdev")?;

    // Create default configuration
    let config = Config::default();
    config.save(&config_path)?;

    // Create .gitignore for .phantomdev
    let gitignore_path = PathBuf::from(".phantomdev/.gitignore");
    std::fs::write(gitignore_path, "*\n")?;

    println!("{}", "✓ PhantomDev initialized successfully!".green());
    println!("  Configuration: {}", config_path.display().to_string().dimmed());
    println!();
    println!("Next steps:");
    println!("  Run {} to scan your changes", "phantomdev scan".cyan());
    println!("  Run {} to see your stealth score", "phantomdev score".cyan());

    Ok(())
}

/// Scan current changes for AI-generated content
fn cmd_scan(files: Vec<String>, verbose: bool) -> Result<()> {
    println!("{}", "Scanning for AI-generated content...".cyan());

    let detector = PhantomDetector::new()?;

    let files_to_scan = if files.is_empty() {
        // Get staged files from git
        get_staged_files()?
    } else {
        files.into_iter().map(PathBuf::from).collect()
    };

    if files_to_scan.is_empty() {
        println!("{}", "No files to scan. Stage some files first.".yellow());
        return Ok(());
    }

    println!("Scanning {} file(s)...", files_to_scan.len());

    for file_path in files_to_scan {
        if let Some(code) = read_code_block(&file_path)? {
            let result = detector.detect(&code)?;

            let status = if result.score.is_likely_ai(0.15) {
                "⚠️  AI DETECTED".red()
            } else {
                "✓ HUMAN-LIKE".green()
            };

            println!("  {} {} - {}", status, file_path.display(), format!("{:.1}%", result.score.ai_probability * 100.0).dimmed());

            if verbose {
                for pattern in &result.patterns {
                    println!("    - {:?} ({:.1}%)", pattern.pattern_type, pattern.confidence * 100.0);
                }
            }
        }
    }

    Ok(())
}

/// Humanize code to match repository style
fn cmd_humanize(files: Vec<String>, entropy: Option<f32>) -> Result<()> {
    println!("{}", "Humanizing code...".cyan());

    let humanizer = PhantomHumanizer::new();
    let repo_path = std::env::current_dir()?;
    let profile = humanizer.learn_style(&repo_path)?;

    println!("Learned style profile:");
    println!("  Naming: {:?}", profile.naming_convention);
    println!("  Comments: {:?}", profile.comment_style);
    println!("  Indentation: {:?}", profile.indentation);

    let files_to_humanize = if files.is_empty() {
        get_staged_files()?
    } else {
        files.into_iter().map(PathBuf::from).collect()
    };

    if files_to_humanize.is_empty() {
        println!("{}", "No files to humanize. Stage some files first.".yellow());
        return Ok(());
    }

    println!("Humanizing {} file(s)...", files_to_humanize.len());

    for file_path in files_to_humanize {
        if let Some(code) = read_code_block(&file_path)? {
            let humanized = humanizer.humanize(&code, &profile)?;
            println!("  ✓ {}", file_path.display());
            // TODO: Write humanized content back to file
        }
    }

    println!("{}", "✓ Humanization complete!".green());

    Ok(())
}

/// Show stealth score for current changes
fn cmd_score(detailed: bool) -> Result<()> {
    println!("{}", "Calculating stealth score...".cyan());

    let detector = PhantomDetector::new()?;
    let files = get_staged_files()?;

    if files.is_empty() {
        println!("{}", "No staged files. Stage some files first.".yellow());
        return Ok(());
    }

    let mut total_score = 0.0;
    let mut file_count = 0;

    for file_path in files {
        if let Some(code) = read_code_block(&file_path)? {
            let result = detector.detect(&code)?;
            total_score += result.score.overall;
            file_count += 1;

            if detailed {
                println!("  {} - {:.1}%", file_path.display(), result.score.overall * 100.0);
            }
        }
    }

    if file_count > 0 {
        let avg_score = total_score / file_count as f32;
        let score_display = format!("{:.1}%", avg_score * 100.0);

        println!();
        println!("Overall Stealth Score: {}", score_display.bold());

        let status = if avg_score > 0.85 {
            "✓ EXCELLENT".green()
        } else if avg_score > 0.70 {
            "⚠️  GOOD".yellow()
        } else {
            "✗ NEEDS IMPROVEMENT".red()
        };

        println!("Status: {}", status);
    }

    Ok(())
}

/// Launch TUI dashboard
fn cmd_dashboard() -> Result<()> {
    println!("{}", "Launching PhantomDev Dashboard...".cyan());

    let mut tui = PhantomTui::new();
    tui.run()?;

    Ok(())
}

/// Configure PhantomDev
fn cmd_config(show: bool, reset: bool) -> Result<()> {
    let config_path = PathBuf::from(".phantomdev/config.toml");

    if reset {
        let config = Config::default();
        config.save(&config_path)?;
        println!("{}", "✓ Configuration reset to defaults".green());
        return Ok(());
    }

    if show || !config_path.exists() {
        let config = Config::load_or_default(&config_path)?;
        println!("Current Configuration:");
        println!("  Detection threshold: {:.0}%", config.detection.threshold * 100.0);
        println!("  Use local models: {}", config.detection.use_local);
        println!("  Use cloud fallback: {}", config.detection.use_cloud_fallback);
        println!("  Auto-humanize: {}", config.humanization.auto_humanize);
        println!("  Entropy level: {:.0}%", config.humanization.entropy_level * 100.0);
        println!("  Jitter enabled: {}", config.jitter.enabled);
    } else {
        println!("Use --show to view current configuration");
        println!("Use --reset to reset to defaults");
    }

    Ok(())
}

/// Get staged files from git
fn get_staged_files() -> Result<Vec<PathBuf>> {
    let output = std::process::Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .filter(|l| !l.is_empty())
            .map(PathBuf::from)
            .collect())
    } else {
        Ok(Vec::new())
    }
}

/// Read a code block from a file
fn read_code_block(path: &PathBuf) -> Result<Option<CodeBlock>> {
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(path)?;
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let language = Language::from_extension(extension);

    let lines: Vec<&str> = content.lines().collect();
    let line_range = if lines.is_empty() {
        (1, 1)
    } else {
        (1, lines.len())
    };

    Ok(Some(CodeBlock {
        path: path.clone(),
        language,
        content,
        line_range,
    }))
}

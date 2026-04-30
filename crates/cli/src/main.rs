//! PhantomDev CLI
//!
//! The main command-line interface for PhantomDev.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use phantomdev_core::{CodeBlock, Config, Detector, Humanizer, Language};
use phantomdev_detector::PhantomDetector;
use phantomdev_humanizer::PhantomHumanizer;
use phantomdev_undercover::UndercoverEngine;
use phantomdev_tui::PhantomTui;

/// PhantomDev - The Adversarial Stylometry Framework for the AI-Augmented Developer
#[derive(Parser)]
#[command(name = "phantomdev")]
#[command(author = "John Varghese (J0X) <john@phantomdev.io>")]
#[command(version = "0.1.0")]
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
    /// Easy install PhantomDev (recommended for new users)
    EasyInstall {
        /// IDE to install skills for
        #[arg(short, long)]
        ide: Option<String>,
    },
    /// Install IDE skills/rules for AI assistants
    InstallSkills {
        /// IDE to install for (claude, cursor, windsurf, antigravity, all)
        #[arg(short, long)]
        ide: String,
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
    /// Undercover mode - transform AI-generated content to human-like patterns
    Undercover {
        /// Transform commit message
        #[arg(short, long)]
        message: Option<String>,
        /// Transform code comments
        #[arg(short, long)]
        comments: bool,
        /// Transform variable names
        #[arg(short, long)]
        variables: bool,
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
        Commands::EasyInstall { ide } => cmd_easy_install(ide),
        Commands::InstallSkills { ide } => cmd_install_skills(ide),
        Commands::Scan { files, verbose } => cmd_scan(files, verbose),
        Commands::Humanize { files, entropy } => cmd_humanize(files, entropy),
        Commands::Undercover { message, comments, variables, entropy } => cmd_undercover(message, comments, variables, entropy),
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
    println!("  Run {} for easy IDE setup", "phantomdev easy-install".cyan());

    Ok(())
}

/// Easy install PhantomDev
fn cmd_easy_install(ide: Option<String>) -> Result<()> {
    println!("{}", "🚀 PhantomDev Easy Install".cyan());
    println!();

    // Initialize PhantomDev
    cmd_init(true)?;

    // Install git hooks
    println!("{}", "Installing git hooks...".cyan());
    let hooks_dir = PathBuf::from("hooks");
    let git_hooks_dir = PathBuf::from(".git/hooks");

    if hooks_dir.exists() {
        for hook in ["pre-commit", "commit-msg"] {
            let src = hooks_dir.join(hook);
            let dst = git_hooks_dir.join(hook);
            if src.exists() {
                std::fs::copy(&src, &dst)?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&dst)?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&dst, perms)?;
                }
                println!("  ✓ Installed {}", hook);
            }
        }
    }

    // Install IDE skills if specified
    if let Some(ide_name) = ide {
        println!();
        println!("{}", "Installing IDE skills...".cyan());
        cmd_install_skills(ide_name)?;
    } else {
        println!();
        println!("To install IDE skills, run:");
        println!("  phantomdev install-skills --ide claude");
        println!("  phantomdev install-skills --ide cursor");
        println!("  phantomdev install-skills --ide windsurf");
        println!("  phantomdev install-skills --ide antigravity");
        println!("  phantomdev install-skills --ide all");
    }

    println!();
    println!("{}", "✓ Easy install complete!".green());
    println!();
    println!("Quick start:");
    println!("  phantomdev scan          # Scan for AI-generated content");
    println!("  phantomdev humanize      # Humanize code");
    println!("  phantomdev score         # Check stealth score");
    println!("  phantomdev dashboard     # Launch TUI dashboard");

    Ok(())
}

/// Install IDE skills
fn cmd_install_skills(ide: String) -> Result<()> {
    let skills_dir = PathBuf::from("skills");

    let ides: Vec<&str> = if ide.to_lowercase() == "all" {
        vec!["claude", "cursor", "windsurf", "antigravity"]
    } else {
        vec![ide.to_lowercase().as_str()]
    };

    for ide_name in ides {
        let skill_file = match ide_name {
            "claude" => "claude-code.md",
            "cursor" => "cursor.md",
            "windsurf" => "windsurf.md",
            "antigravity" => "antigravity.md",
            _ => {
                println!("⚠️  Unknown IDE: {}", ide_name);
                continue;
            }
        };

        let src = skills_dir.join(skill_file);
        if !src.exists() {
            println!("⚠️  Skill file not found: {}", skill_file);
            continue;
        }

        let content = std::fs::read_to_string(&src)?;

        // Determine destination based on IDE
        let dst = match ide_name {
            "claude" => PathBuf::from(".claude/skills/phantomdev.md"),
            "cursor" => PathBuf::from(".cursor/rules/phantomdev.md"),
            "windsurf" => PathBuf::from(".windsurf/rules/phantomdev.md"),
            "antigravity" => PathBuf::from(".antigravity/rules/phantomdev.md"),
            _ => continue,
        };

        // Create destination directory
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write skill file
        std::fs::write(&dst, content)?;
        println!("  ✓ Installed {} skill", ide_name);
    }

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
fn cmd_humanize(files: Vec<String>, _entropy: Option<f32>) -> Result<()> {
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
            let _humanized = humanizer.humanize(&code, &profile)?;
            println!("  ✓ {}", file_path.display());
            // TODO: Write humanized content back to file
        }
    }

    println!("{}", "✓ Humanization complete!".green());

    Ok(())
}

/// Undercover mode - transform AI-generated content to human-like patterns
fn cmd_undercover(message: Option<String>, comments: bool, variables: bool, _entropy: Option<f32>) -> Result<()> {
    println!("{}", "🕵️ PhantomDev Undercover Mode".cyan());
    println!();

    let engine = UndercoverEngine::new();

    // Transform commit message if provided
    if let Some(msg) = message {
        println!("Original: {}", msg.dimmed());
        let transformed = engine.transform_commit_message(&msg);
        println!("Transformed: {}", transformed.green());
        println!();
    }

    // Transform code if requested
    if comments || variables {
        let files = get_staged_files()?;

        if files.is_empty() {
            println!("{}", "No staged files. Stage some files first.".yellow());
            return Ok(());
        }

        println!("Transforming {} file(s)...", files.len());

        for file_path in files {
            if let Some(code) = read_code_block(&file_path)? {
                let mut transformed = code.content.clone();

                if comments {
                    transformed = engine.transform_comments(&transformed);
                }

                if variables {
                    transformed = engine.transform_variable_names(&transformed);
                }

                // Write transformed content
                std::fs::write(&file_path, transformed)?;
                println!("  ✓ {}", file_path.display());

                // Stage the file
                std::process::Command::new("git")
                    .args(["add", file_path.to_str().unwrap()])
                    .output()?;
            }
        }

        println!();
        println!("{}", "✓ Undercover transformation complete!".green());
    }

    // Check for banned words in all staged files
    let files = get_staged_files()?;
    let mut total_banned = 0;

    for file_path in files {
        if let Some(code) = read_code_block(&file_path)? {
            let banned = engine.find_banned_words(&code.content);
            if !banned.is_empty() {
                println!("⚠️  {} contains banned words: {}", file_path.display(), banned.join(", ").dimmed());
                total_banned += banned.len();
            }
        }
    }

    if total_banned > 0 {
        println!();
        println!("💡 Run 'phantomdev humanize' to fix banned words");
    }

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

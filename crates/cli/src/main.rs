//! PhantomDev CLI - Simplified UX
//!
//! A conversational, easy-to-use CLI for PhantomDev.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use phantomdev_core::{CodeBlock, Config, Detector, Humanizer, Language};
use phantomdev_detector::PhantomDetector;
use phantomdev_humanizer::PhantomHumanizer;
use phantomdev_undercover::UndercoverEngine;
use phantomdev_tui::PhantomTui;
use std::path::PathBuf;
use std::io::{self, Write};

/// PhantomDev - Make your AI code look human
#[derive(Parser)]
#[command(name = "phantomdev")]
#[command(author = "John Varghese (J0X)")]
#[command(version = "0.1.0")]
#[command(about = "Make your AI-generated code look human-written", long_about = None)]
#[command(after_help = "
Quick Start:
  phantomdev              # Check status and get suggestions
  phantomdev fix          # Auto-fix AI patterns in staged files
  phantomdev scan         # See what's detected
  phantomdev dashboard    # Launch visual dashboard

Learn more: https://john-varghese-eh.github.io/PhantomDev/
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fix AI patterns in your code (recommended)
    Fix {
        /// Files to fix (default: staged files)
        #[arg(short, long)]
        files: Vec<String>,
        /// Show what would be changed without applying
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Scan for AI-generated content
    Scan {
        /// Files to scan (default: staged files)
        #[arg(short, long)]
        files: Vec<String>,
    },
    /// Check your stealth score
    Score {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },
    /// Launch visual dashboard
    Dashboard,
    /// Configure settings
    Config {
        /// Show current settings
        #[arg(short, long)]
        show: bool,
        /// Reset to defaults
        #[arg(short, long)]
        reset: bool,
    },
    /// Initialize in current directory
    Init {
        /// Force reinitialize
        #[arg(short, long)]
        force: bool,
    },
    /// Install IDE integration
    Install {
        /// IDE to install for (claude, cursor, windsurf, antigravity, all)
        #[arg(short = 'i', long)]
        ide: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no command, show interactive status
    let command = cli.command.unwrap_or(Commands::Score { detailed: false });

    match command {
        Commands::Fix { files, dry_run } => cmd_fix(files, dry_run),
        Commands::Scan { files } => cmd_scan(files),
        Commands::Score { detailed } => cmd_score(detailed),
        Commands::Dashboard => cmd_dashboard(),
        Commands::Config { show, reset } => cmd_config(show, reset),
        Commands::Init { force } => cmd_init(force),
        Commands::Install { ide } => cmd_install(ide),
    }
}

/// Fix AI patterns in your code
fn cmd_fix(files: Vec<String>, dry_run: bool) -> Result<()> {
    ensure_initialized()?;

    println!("{}", "🔍 Scanning for AI patterns...".cyan());
    println!();

    let detector = PhantomDetector::new()?;
    let files_to_fix = if files.is_empty() {
        get_staged_files()?
    } else {
        files.into_iter().map(PathBuf::from).collect()
    };

    if files_to_fix.is_empty() {
        println!("{}", "ℹ️  No files to fix. Stage some files first.".dimmed());
        println!();
        println!("Try: {}", "git add <files>".cyan());
        return Ok(());
    }

    let mut issues_found = 0;
    let mut files_with_issues = Vec::new();

    for file_path in &files_to_fix {
        if let Some(code) = read_code_block(file_path)? {
            let result = detector.detect(&code)?;

            if result.score.is_likely_ai(0.15) {
                issues_found += 1;
                files_with_issues.push((file_path.clone(), result));
                println!("  ⚠️  {} - {:.0}% AI detected", file_path.display(), result.score.ai_probability * 100.0);
            } else {
                println!("  ✓ {} - Looks good", file_path.display());
            }
        }
    }

    println!();

    if issues_found == 0 {
        println!("{}", "✨ Your code looks great! No AI patterns detected.".green());
        return Ok(());
    }

    if dry_run {
        println!("{}", "📋 Dry run - showing what would be fixed:".yellow());
        for (path, result) in &files_with_issues {
            println!("  • {} - {:.0}% AI", path.display(), result.score.ai_probability * 100.0);
        }
        println!();
        println!("Run {} to apply fixes", "phantomdev fix".cyan());
        return Ok(());
    }

    println!("{}", "🔧 Fixing {} file(s)...".cyan(), issues_found);

    let humanizer = PhantomHumanizer::new();
    let repo_path = std::env::current_dir()?;
    let profile = humanizer.learn_style(&repo_path)?;

    let mut fixed_count = 0;
    for (file_path, _) in &files_with_issues {
        if let Some(code) = read_code_block(file_path)? {
            let humanized = humanizer.humanize(&code, &profile)?;
            std::fs::write(file_path, humanized)?;
            println!("  ✓ {}", file_path.display());
            fixed_count += 1;

            // Re-stage the file
            let _ = std::process::Command::new("git")
                .args(["add", file_path.to_str().unwrap()])
                .output();
        }
    }

    println!();
    println!("{}", "✨ Fixed {} file(s)!".green(), fixed_count);
    println!();
    println!("Next: {}", "git commit".cyan());
}

/// Scan for AI-generated content
fn cmd_scan(files: Vec<String>) -> Result<()> {
    ensure_initialized()?;

    println!("{}", "🔍 Scanning for AI patterns...".cyan());
    println!();

    let detector = PhantomDetector::new()?;
    let files_to_scan = if files.is_empty() {
        get_staged_files()?
    } else {
        files.into_iter().map(PathBuf::from).collect()
    };

    if files_to_scan.is_empty() {
        println!("{}", "ℹ️  No files to scan. Stage some files first.".dimmed());
        println!();
        println!("Try: {}", "git add <files>".cyan());
        return Ok(());
    }

    let mut ai_count = 0;

    for file_path in &files_to_scan {
        if let Some(code) = read_code_block(file_path)? {
            let result = detector.detect(&code)?;

            if result.score.is_likely_ai(0.15) {
                ai_count += 1;
                println!("  ⚠️  {} - {:.0}% AI", file_path.display(), result.score.ai_probability * 100.0);
            } else {
                println!("  ✓ {} - Looks human", file_path.display());
            }
        }
    }

    println!();

    if ai_count > 0 {
        println!("Found AI patterns in {} file(s).", ai_count);
        println!("Run {} to fix them", "phantomdev fix".cyan());
    } else {
        println!("{}", "✨ No AI patterns detected!".green());
    }
}

/// Check stealth score
fn cmd_score(detailed: bool) -> Result<()> {
    ensure_initialized()?;

    println!("{}", "📊 Checking stealth score...".cyan());
    println!();

    let detector = PhantomDetector::new()?;
    let files = get_staged_files()?;

    if files.is_empty() {
        println!("{}", "ℹ️  No staged files. Stage some files first.".dimmed());
        println!();
        println!("Try: {}", "git add <files>".cyan());
        return Ok(());
    }

    let mut total_score = 0.0;
    let mut file_count = 0;

    for file_path in &files {
        if let Some(code) = read_code_block(file_path)? {
            let result = detector.detect(&code)?;
            total_score += result.score.overall;
            file_count += 1;

            if detailed {
                let status = if result.score.is_likely_ai(0.15) {
                    "⚠️".red()
                } else {
                    "✓".green()
                };
                println!("  {} {} - {:.0}%", status, file_path.display(), result.score.overall * 100.0);
            }
        }
    }

    if file_count > 0 {
        let avg_score = total_score / file_count as f32;
        let stealth_score = 1.0 - avg_score; // Invert: higher = more stealthy
        let score_display = format!("{:.0}%", stealth_score * 100.0);

        println!();
        println!("Stealth Score: {}", score_display.bold());

        let (emoji, status, color) = if stealth_score > 0.85 {
            ("✨", "EXCELLENT", "green")
        } else if stealth_score > 0.70 {
            ("👍", "GOOD", "yellow")
        } else {
            ("⚠️", "NEEDS WORK", "red")
        };

        println!("{} Status: {}", emoji, status.color(color));

        if stealth_score < 0.85 {
            println!();
            println!("Run {} to improve your score", "phantomdev fix".cyan());
        }
    }
}

/// Launch dashboard
fn cmd_dashboard() -> Result<()> {
    println!("{}", "🚀 Launching dashboard...".cyan());
    let mut tui = PhantomTui::new();
    tui.run()?;
    Ok(())
}

/// Configure settings
fn cmd_config(show: bool, reset: bool) -> Result<()> {
    let config_path = PathBuf::from(".phantomdev/config.toml");

    if reset {
        let config = Config::default();
        config.save(&config_path)?;
        println!("{}", "✓ Settings reset to defaults".green());
        return Ok(());
    }

    let config = Config::load_or_default(&config_path)?;

    println!("{}", "Current Settings:".cyan());
    println!("  Detection threshold: {:.0}%", config.detection.threshold * 100.0);
    println!("  Auto-humanize: {}", config.humanization.auto_humanize);
    println!("  Entropy level: {:.0}%", config.humanization.entropy_level * 100.0);
}

/// Initialize
fn cmd_init(force: bool) -> Result<()> {
    let config_path = PathBuf::from(".phantomdev/config.toml");

    if config_path.exists() && !force {
        println!("{}", "✓ Already initialized".green());
        return Ok(());
    }

    std::fs::create_dir_all(".phantomdev")?;
    let config = Config::default();
    config.save(&config_path)?;

    println!("{}", "✓ Initialized!".green());
    println!();
    println!("Ready to go! Run {} to check your code", "phantomdev".cyan());
}

/// Install IDE integration
fn cmd_install(ide: Option<String>) -> Result<()> {
    let skills_dir = PathBuf::from("skills");

    if !skills_dir.exists() {
        println!("{}", "⚠️  Skills directory not found. Are you in the PhantomDev repository?".yellow());
        return Ok(());
    }

    let ide_name = ide.unwrap_or_else(|| {
        // Interactive selection
        println!("{}", "Select IDE to install:".cyan());
        println!("  1) Claude Code");
        println!("  2) Cursor");
        println!("  3) Windsurf");
        println!("  4) Antigravity");
        println!("  5) All");
        print!("Enter choice (1-5): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => "claude",
            "2" => "cursor",
            "3" => "windsurf",
            "4" => "antigravity",
            "5" => "all",
            _ => "all",
        }
        .to_string()
    });

    let ide_lower = ide_name.to_lowercase();
    let ides: Vec<&str> = if ide_lower == "all" {
        vec!["claude", "cursor", "windsurf", "antigravity"]
    } else {
        vec![ide_lower.as_str()]
    };

    println!();
    println!("{}", "Installing IDE integration...".cyan());

    for ide_name in ides {
        let skill_file = match ide_name {
            "claude" => "claude-code.md",
            "cursor" => "cursor.md",
            "windsurf" => "windsurf.md",
            "antigravity" => "antigravity.md",
            _ => {
                println!("  ⚠️  Unknown IDE: {}", ide_name);
                continue;
            }
        };

        let src = skills_dir.join(skill_file);
        if !src.exists() {
            println!("  ⚠️  Skill file not found: {}", skill_file);
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
        println!("  ✓ Installed {} integration", ide_name);
    }

    println!();
    println!("{}", "✓ Installation complete!".green());
    println!();
    println!("Restart your IDE to apply changes.");

    Ok(())
}

/// Ensure PhantomDev is initialized
fn ensure_initialized() -> Result<()> {
    let config_path = PathBuf::from(".phantomdev/config.toml");
    if !config_path.exists() {
        println!("{}", "📝 Setting up PhantomDev...".dimmed());
        cmd_init(false)?;
        println!();
    }
    Ok(())
}

/// Get staged files
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

/// Read code from file
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
    let line_range = if lines.is_empty() { (1, 1) } else { (1, lines.len()) };

    Ok(Some(CodeBlock {
        path: path.clone(),
        language,
        content,
        line_range,
    }))
}

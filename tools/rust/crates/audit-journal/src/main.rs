mod index;
mod journal;
mod templates;
mod validate;
mod worklog;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "audit-journal")]
#[command(about = "Journal and worklog management for the audit orchestrator")]
struct Cli {
    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new journal entry
    Create {
        /// Date for the entry (YYYY-MM-DD)
        #[arg(long)]
        date: String,

        /// Title for the journal entry
        #[arg(long, default_value = "Audit Cycle")]
        title: String,
    },

    /// Create a new worklog entry
    Worklog {
        /// Date for the entry (YYYY-MM-DD)
        #[arg(long)]
        date: String,

        /// Cycle number
        #[arg(long)]
        cycle: u32,

        /// Brief summary line
        #[arg(long, default_value = "")]
        summary: String,
    },

    /// Rebuild JOURNAL.md index from docs/journal/ entries
    Index {
        /// Check mode: report issues without writing
        #[arg(long)]
        check: bool,
    },

    /// List existing entries
    List {
        /// Show only journal entries
        #[arg(long)]
        journal: bool,

        /// Show only worklog entries
        #[arg(long)]
        worklog: bool,
    },

    /// Validate entries for formatting issues
    Validate {
        /// Attempt to fix issues automatically
        #[arg(long)]
        fix: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let root = &cli.repo_root;

    let result = match cli.command {
        Commands::Create { date, title } => {
            journal::create(root, &date, &title)
        }
        Commands::Worklog {
            date,
            cycle,
            summary,
        } => worklog::create(root, &date, cycle, &summary),
        Commands::Index { check } => index::rebuild(root, check),
        Commands::List { journal, worklog } => {
            if worklog {
                list_entries(root, "worklog")
            } else if journal {
                list_entries(root, "journal")
            } else {
                list_entries(root, "both")
            }
        }
        Commands::Validate { fix } => validate::run(root, fix),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn list_entries(root: &PathBuf, kind: &str) -> Result<(), String> {
    let show_journal = kind == "journal" || kind == "both";
    let show_worklog = kind == "worklog" || kind == "both";

    if show_journal {
        let journal_dir = root.join("docs/journal");
        if journal_dir.is_dir() {
            let mut entries = list_md_files(&journal_dir)?;
            entries.sort();
            entries.reverse();
            if !entries.is_empty() {
                println!("Journal entries:");
                for entry in &entries {
                    let title = extract_title(&journal_dir.join(entry));
                    println!("  {entry}  {title}");
                }
            } else {
                println!("Journal entries: (none)");
            }
        } else {
            println!("Journal entries: (none)");
        }
    }

    if show_worklog {
        if show_journal {
            println!();
        }
        let worklog_dir = root.join("docs/worklog");
        if worklog_dir.is_dir() {
            let mut entries = list_md_files(&worklog_dir)?;
            entries.sort();
            entries.reverse();
            if !entries.is_empty() {
                println!("Worklog entries:");
                for entry in &entries {
                    let title = extract_title(&worklog_dir.join(entry));
                    println!("  {entry}  {title}");
                }
            } else {
                println!("Worklog entries: (none)");
            }
        } else {
            println!("Worklog entries: (none)");
        }
    }

    Ok(())
}

fn list_md_files(dir: &std::path::Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {e}", dir.display()))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".md") {
            files.push(name);
        }
    }
    Ok(files)
}

fn extract_title(path: &std::path::Path) -> String {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return trimmed[2..].to_string();
        }
    }
    String::new()
}

use regex::Regex;
use std::path::Path;

pub fn run(root: &Path, fix: bool) -> Result<(), String> {
    let mut issues_found = 0;
    let mut issues_fixed = 0;

    // Validate journal entries
    let journal_dir = root.join("docs/journal");
    if journal_dir.is_dir() {
        let (found, fixed) = validate_directory(&journal_dir, "journal", fix)?;
        issues_found += found;
        issues_fixed += fixed;
    }

    // Validate worklog entries
    let worklog_dir = root.join("docs/worklog");
    if worklog_dir.is_dir() {
        let (found, fixed) = validate_directory(&worklog_dir, "worklog", fix)?;
        issues_found += found;
        issues_fixed += fixed;
    }

    // Check JOURNAL.md index
    let index_path = root.join("JOURNAL.md");
    if index_path.exists() {
        let content = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read JOURNAL.md: {e}"))?;

        if !content.contains("Auto-generated") {
            println!("JOURNAL.md: missing auto-generated header (stale placeholder?)");
            issues_found += 1;
        }
    } else {
        println!("JOURNAL.md: file does not exist");
        issues_found += 1;
    }

    if issues_found == 0 {
        println!("All entries valid.");
    } else if fix {
        println!(
            "\n{issues_found} issue(s) found, {issues_fixed} fixed."
        );
    } else {
        println!("\n{issues_found} issue(s) found. Run with --fix to attempt repairs.");
    }

    Ok(())
}

fn validate_directory(
    dir: &Path,
    kind: &str,
    fix: bool,
) -> Result<(usize, usize), String> {
    let mut issues_found = 0;
    let mut issues_fixed = 0;

    let read_dir = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read {}: {e}", dir.display()))?;

    let date_re =
        Regex::new(r"^\d{4}-\d{2}-\d{2}").expect("Invalid regex");
    let link_re =
        Regex::new(r"\[(?:#|main#|qc#)\d+\]\(https://github\.com/EvaLok/").expect("Invalid regex");
    let bare_ref_re =
        Regex::new(r"(?:^|\s)(?:#|main#|qc#)\d+(?:\s|$|[,.\)])").expect("Invalid regex");

    for entry in read_dir {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if !filename.ends_with(".md") {
            continue;
        }

        let content = std::fs::read_to_string(entry.path())
            .map_err(|e| format!("Failed to read {filename}: {e}"))?;

        let label = format!("docs/{kind}/{filename}");

        // Check 1: filename should start with a date
        if !date_re.is_match(&filename) {
            println!("{label}: filename does not start with a date (YYYY-MM-DD)");
            issues_found += 1;
        }

        // Check 2: must have a top-level heading
        let has_title = content.lines().any(|l| l.trim().starts_with("# "));
        if !has_title {
            println!("{label}: missing top-level heading (# Title)");
            issues_found += 1;
        }

        // Check 3: check for bare issue references (should be clickable links)
        let mut has_bare_refs = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Skip HTML comments
            if trimmed.starts_with("<!--") {
                continue;
            }
            if bare_ref_re.is_match(line) && !link_re.is_match(line) {
                println!(
                    "{label}:{}: bare issue reference (should be a clickable link)",
                    i + 1
                );
                has_bare_refs = true;
                issues_found += 1;
            }
        }

        // Check 4: entry should have required sections
        let required_sections: &[&str] = if kind == "journal" {
            &["## Summary"]
        } else {
            &["## Steps Performed"]
        };

        for section in required_sections {
            if !content.contains(section) {
                println!("{label}: missing required section '{section}'");
                issues_found += 1;
            }
        }

        // Fix: add missing sections if --fix
        if fix && !has_bare_refs {
            let mut modified = content.clone();
            let mut did_fix = false;

            for section in required_sections {
                if !modified.contains(section) {
                    modified.push_str(&format!("\n{section}\n\n<!-- TODO -->\n"));
                    did_fix = true;
                    issues_fixed += 1;
                }
            }

            if did_fix {
                std::fs::write(entry.path(), &modified)
                    .map_err(|e| format!("Failed to write {filename}: {e}"))?;
                println!("{label}: fixed missing sections");
            }
        }
    }

    Ok((issues_found, issues_fixed))
}

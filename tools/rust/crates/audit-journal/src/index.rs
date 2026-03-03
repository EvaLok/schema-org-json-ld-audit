use crate::templates;
use std::collections::BTreeMap;
use std::path::Path;

/// Entry metadata extracted from a journal file
struct JournalEntry {
    date: String,
    title: String,
    tags: String,
    filename: String,
}

pub fn rebuild(root: &Path, check: bool) -> Result<(), String> {
    let journal_dir = root.join("docs/journal");

    if !journal_dir.is_dir() {
        if check {
            println!("No docs/journal/ directory found.");
            return Ok(());
        }
        // Create empty index
        let content = format!("{}\n*No entries yet.*\n", templates::index_header());
        let index_path = root.join("JOURNAL.md");
        std::fs::write(&index_path, &content)
            .map_err(|e| format!("Failed to write JOURNAL.md: {e}"))?;
        println!("Rebuilt JOURNAL.md (empty)");
        return Ok(());
    }

    let entries = scan_journal_entries(&journal_dir)?;

    if check {
        let index_path = root.join("JOURNAL.md");
        if !index_path.exists() {
            println!("JOURNAL.md does not exist — run `audit-journal index` to create it.");
            return Ok(());
        }
        let current = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read JOURNAL.md: {e}"))?;
        let expected = build_index_content(&entries);
        if current == expected {
            println!("JOURNAL.md is up to date.");
        } else {
            println!("JOURNAL.md is stale — run `audit-journal index` to rebuild.");
        }
        return Ok(());
    }

    let content = build_index_content(&entries);
    let index_path = root.join("JOURNAL.md");
    std::fs::write(&index_path, &content)
        .map_err(|e| format!("Failed to write JOURNAL.md: {e}"))?;

    println!("Rebuilt JOURNAL.md ({} entries)", entries.len());
    Ok(())
}

fn scan_journal_entries(dir: &Path) -> Result<Vec<JournalEntry>, String> {
    let read_dir = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read docs/journal/: {e}"))?;

    let mut entries = Vec::new();

    for entry in read_dir {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if !filename.ends_with(".md") {
            continue;
        }

        let content = std::fs::read_to_string(entry.path())
            .map_err(|e| format!("Failed to read {filename}: {e}"))?;

        let date = filename.trim_end_matches(".md").to_string();
        let title = extract_title(&content);
        let tags = extract_tags(&content);

        entries.push(JournalEntry {
            date,
            title,
            tags,
            filename,
        });
    }

    // Sort reverse-chronological
    entries.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(entries)
}

fn extract_title(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            return trimmed[2..].to_string();
        }
    }
    "Untitled".to_string()
}

fn extract_tags(content: &str) -> String {
    let mut in_tags_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "## Tags" {
            in_tags_section = true;
            continue;
        }
        if in_tags_section {
            if trimmed.starts_with("## ") {
                break;
            }
            if !trimmed.is_empty() && !trimmed.starts_with("<!--") {
                return trimmed.to_string();
            }
        }
    }

    String::new()
}

fn build_index_content(entries: &[JournalEntry]) -> String {
    let mut content = String::from(templates::index_header());

    if entries.is_empty() {
        content.push_str("\n*No entries yet.*\n");
        return content;
    }

    // Group by month (YYYY-MM)
    let mut by_month: BTreeMap<String, Vec<&JournalEntry>> = BTreeMap::new();
    for entry in entries {
        let month = if entry.date.len() >= 7 {
            entry.date[..7].to_string()
        } else {
            "Unknown".to_string()
        };
        by_month.entry(month).or_default().push(entry);
    }

    // Output in reverse-chronological order (BTreeMap is ascending, so reverse)
    let months: Vec<_> = by_month.keys().cloned().collect();
    for month in months.iter().rev() {
        content.push_str(&format!("\n## {month}\n\n"));
        content.push_str("| Date | Title | Tags |\n");
        content.push_str("|------|-------|------|\n");

        if let Some(entries) = by_month.get(month) {
            for entry in entries {
                content.push_str(&format!(
                    "| [{}](docs/journal/{}) | {} | {} |\n",
                    entry.date, entry.filename, entry.title, entry.tags
                ));
            }
        }
    }

    content
}

use crate::templates;
use std::path::Path;

pub fn create(root: &Path, date: &str, title: &str) -> Result<(), String> {
    validate_date(date)?;

    let journal_dir = root.join("docs/journal");
    std::fs::create_dir_all(&journal_dir)
        .map_err(|e| format!("Failed to create docs/journal/: {e}"))?;

    let filename = format!("{date}.md");
    let filepath = journal_dir.join(&filename);

    if filepath.exists() {
        return Err(format!("Journal entry already exists: docs/journal/{filename}"));
    }

    let content = templates::journal_entry(date, title);
    std::fs::write(&filepath, &content)
        .map_err(|e| format!("Failed to write {}: {e}", filepath.display()))?;

    println!("Created docs/journal/{filename}");

    // Auto-rebuild index after creating an entry
    crate::index::rebuild(root, false)?;

    Ok(())
}

fn validate_date(date: &str) -> Result<(), String> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid date format '{date}': expected YYYY-MM-DD"));
    }

    let year: u32 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid year in date '{date}'"))?;
    let month: u32 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid month in date '{date}'"))?;
    let day: u32 = parts[2]
        .parse()
        .map_err(|_| format!("Invalid day in date '{date}'"))?;

    if year < 2024 || year > 2100 {
        return Err(format!("Year {year} out of range (2024-2100)"));
    }
    if !(1..=12).contains(&month) {
        return Err(format!("Month {month} out of range (1-12)"));
    }
    if !(1..=31).contains(&day) {
        return Err(format!("Day {day} out of range (1-31)"));
    }

    Ok(())
}

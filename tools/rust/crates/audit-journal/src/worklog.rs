use crate::templates;
use std::path::Path;

pub fn create(root: &Path, date: &str, cycle: u32, summary: &str) -> Result<(), String> {
    validate_date(date)?;

    let worklog_dir = root.join("docs/worklog");
    std::fs::create_dir_all(&worklog_dir)
        .map_err(|e| format!("Failed to create docs/worklog/: {e}"))?;

    let filename = format!("{date}-cycle{cycle}.md");
    let filepath = worklog_dir.join(&filename);

    if filepath.exists() {
        return Err(format!(
            "Worklog entry already exists: docs/worklog/{filename}"
        ));
    }

    let content = templates::worklog_entry(date, cycle, summary);
    std::fs::write(&filepath, &content)
        .map_err(|e| format!("Failed to write {}: {e}", filepath.display()))?;

    println!("Created docs/worklog/{filename}");

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

use std::fs;
use std::path::Path;

pub fn detect_packages() -> Option<String> {
    let mut counts = Vec::new();

    let brew_dirs = [
        "/opt/homebrew/Cellar",
        "/usr/local/Cellar",
        "/home/linuxbrew/.linuxbrew/Cellar",
    ];
    for b in &brew_dirs {
        if let Ok(entries) = fs::read_dir(b) {
            let count = entries.flatten().filter(|e| e.path().is_dir()).count();
            if count > 0 {
                counts.push(format!("{count} (brew)"));
                break;
            }
        }
    }

    let port_db = Path::new("/opt/local/var/macports/registry/registry.db");
    if port_db.exists() {
        if let Ok(meta) = fs::metadata(port_db) {
            if meta.len() > 0 {
                if let Ok(entries) = fs::read_dir("/opt/local/var/macports/software") {
                    let count = entries.flatten().filter(|e| e.path().is_dir()).count();
                    if count > 0 {
                        counts.push(format!("{count} (macports)"));
                    }
                }
            }
        }
    }

    if counts.is_empty() { None } else { Some(counts.join(", ")) }
}

use crate::utils::clean;
use std::env;
use std::path::Path;

pub fn detect_shell() -> Option<String> {
    if let Ok(shell) = env::var("SHELL") {
        let name = Path::new(&shell)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(&shell);
        return Some(clean(name));
    }
    if env::var("PSModulePath").is_ok() {
        if let Ok(p) = env::var("POWERSHELL_DISTRIBUTION_CHANNEL") {
            return Some(format!("pwsh ({})", clean(&p)));
        }
        return Some("PowerShell".into());
    }
    if let Ok(comspec) = env::var("ComSpec").or_else(|_| env::var("COMSPEC")) {
        let name = Path::new(&comspec)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("cmd.exe");
        return Some(clean(name));
    }
    Some("cmd.exe".into())
}

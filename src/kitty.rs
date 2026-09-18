use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Check if running inside Kitty terminal.
pub fn is_kitty_terminal() -> bool {
    if env::var("KITTY_WINDOW_ID").is_ok() {
        return true;
    }
    if env::var("TERM").as_deref() == Ok("xterm-kitty") {
        return true;
    }
    if env::var("TERM_PROGRAM")
        .map(|p| p.eq_ignore_ascii_case("kitty"))
        .unwrap_or(false)
    {
        return true;
    }
    false
}

/// Check if the terminal supports the Kitty graphics protocol (Kitty, WezTerm, Ghostty, foot).
pub fn supports_kitty_graphics() -> bool {
    if is_kitty_terminal() {
        return true;
    }
    if let Ok(prog) = env::var("TERM_PROGRAM") {
        let p = prog.to_lowercase();
        if p == "wezterm" || p == "ghostty" || p == "foot" {
            return true;
        }
    }
    if let Ok(term) = env::var("LC_TERMINAL") {
        let t = term.to_lowercase();
        if t.contains("kitty") || t.contains("wezterm") {
            return true;
        }
    }
    false
}

/// Check if a path looks like an image file based on extension or existence.
pub fn is_image_path(path_str: &str) -> bool {
    let lower = path_str.to_lowercase();
    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".webp")
        || lower.ends_with(".gif")
        || lower.ends_with(".bmp")
    {
        return true;
    }

    let p = crate::config::expand_tilde(path_str);
    let Ok(mut f) = fs::File::open(&p) else {
        return false;
    };
    let Ok(meta) = f.metadata() else {
        return false;
    };
    if !meta.is_file() {
        return false;
    }
    use std::io::Read;
    let mut buf = [0u8; 16];
    if let Ok(n) = f.read(&mut buf) {
        if n >= 8 && buf.starts_with(b"\x89PNG\r\n\x1a\n") {
            return true;
        }
        if n >= 3 && buf.starts_with(&[0xff, 0xd8, 0xff]) {
            return true;
        }
        if n >= 6 && (buf.starts_with(b"GIF87a") || buf.starts_with(b"GIF89a")) {
            return true;
        }
        if n >= 12 && buf.starts_with(b"RIFF") && &buf[8..12] == b"WEBP" {
            return true;
        }
    }
    false
}

/// Base64 encoding implementation in pure Rust (zero external dependencies).
pub fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        result.push(CHARS[(b0 >> 2) as usize] as char);
        result.push(CHARS[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(b2 & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Probe pixel dimensions (width, height) from image headers without decoding pixel buffers.
pub fn probe_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    let bytes = fs::read(path).ok()?;
    probe_dimensions_from_bytes(&bytes)
}

pub fn probe_dimensions_from_bytes(bytes: &[u8]) -> Option<(u32, u32)> {
    // PNG: \x89PNG\r\n\x1a\n
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") && bytes.len() >= 24 {
        let w = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
        let h = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
        return Some((w, h));
    }
    // JPEG: 0xFF, 0xD8
    if bytes.starts_with(&[0xff, 0xd8]) {
        let mut i = 2;
        while i + 4 < bytes.len() {
            if bytes[i] != 0xff {
                i += 1;
                continue;
            }
            let marker = bytes[i + 1];
            if marker == 0xd9 || marker == 0xda {
                break;
            }
            let len = u16::from_be_bytes([bytes[i + 2], bytes[i + 3]]) as usize;
            if (0xc0..=0xc3).contains(&marker) && i + 8 < bytes.len() {
                let h = u16::from_be_bytes([bytes[i + 5], bytes[i + 6]]) as u32;
                let w = u16::from_be_bytes([bytes[i + 7], bytes[i + 8]]) as u32;
                return Some((w, h));
            }
            i += 2 + len;
        }
    }
    // GIF: GIF87a or GIF89a
    if (bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a")) && bytes.len() >= 10 {
        let w = u16::from_le_bytes([bytes[6], bytes[7]]) as u32;
        let h = u16::from_le_bytes([bytes[8], bytes[9]]) as u32;
        return Some((w, h));
    }
    // WEBP: RIFF....WEBP
    if bytes.starts_with(b"RIFF") && bytes.len() >= 30 && &bytes[8..12] == b"WEBP" {
        if &bytes[12..16] == b"VP8 " && bytes.len() >= 30 {
            let w = (u16::from_le_bytes([bytes[26], bytes[27]]) & 0x3fff) as u32;
            let h = (u16::from_le_bytes([bytes[28], bytes[29]]) & 0x3fff) as u32;
            return Some((w, h));
        } else if &bytes[12..16] == b"VP8L" && bytes.len() >= 25 {
            let b1 = bytes[21];
            let b2 = bytes[22];
            let b3 = bytes[23];
            let b4 = bytes[24];
            let w = 1 + (((b2 as u32 & 0x3f) << 8) | b1 as u32);
            let h = 1
                + ((((b4 as u32 & 0x0f) << 10) | ((b3 as u32) << 2) | ((b2 as u32) >> 6)) & 0x3fff);
            return Some((w, h));
        } else if &bytes[12..16] == b"VP8X" && bytes.len() >= 30 {
            let w = 1 + (bytes[24] as u32 | ((bytes[25] as u32) << 8) | ((bytes[26] as u32) << 16));
            let h = 1 + (bytes[27] as u32 | ((bytes[28] as u32) << 8) | ((bytes[29] as u32) << 16));
            return Some((w, h));
        }
    }
    None
}

/// Calculate terminal cell dimensions from pixel dimensions and user overrides.
/// Terminal character cells are roughly 2:1 height to width (aspect ratio ~2.0).
pub fn calculate_cell_dimensions(
    px_dims: Option<(u32, u32)>,
    req_w: Option<usize>,
    req_h: Option<usize>,
) -> (usize, usize) {
    if let (Some(w), Some(h)) = (req_w, req_h) {
        return (w, h);
    }

    let aspect = if let Some((pw, ph)) = px_dims {
        if ph > 0 {
            (pw as f64 / ph as f64) * 2.0
        } else {
            2.0
        }
    } else {
        2.0
    };

    if let Some(w) = req_w {
        let h = (w as f64 / aspect).round().max(1.0) as usize;
        return (w, h);
    }

    if let Some(h) = req_h {
        let w = (h as f64 * aspect).round().max(1.0) as usize;
        return (w, h);
    }

    // Default target height is ~15 cells
    let default_h = 15;
    let calc_w = (default_h as f64 * aspect).round().clamp(18.0, 42.0) as usize;
    let calc_h = (calc_w as f64 / aspect).round().clamp(8.0, 24.0) as usize;
    (calc_w, calc_h)
}

/// Generate the Kitty Graphics Protocol escape sequence to display an image.
pub fn generate_kitty_escape(
    path: &Path,
    width: usize,
    height: usize,
    direct: bool,
) -> Result<String, String> {
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if !abs_path.exists() {
        return Err(format!("image file does not exist: {}", abs_path.display()));
    }

    let in_tmux = env::var("TERM")
        .map(|t| t.starts_with("screen") || t.starts_with("tmux"))
        .unwrap_or(false);

    let mut out = String::new();

    if direct {
        // Direct transmission (t=d): payload is base64 of file contents
        let file_bytes = fs::read(&abs_path)
            .map_err(|e| format!("failed to read image file {}: {e}", abs_path.display()))?;
        let b64 = base64_encode(&file_bytes);

        // Chunk in 4096-byte segments per Kitty Graphics Protocol
        const CHUNK_SIZE: usize = 4096;
        let total_chunks = b64.len().div_ceil(CHUNK_SIZE);

        for (i, chunk) in b64.as_bytes().chunks(CHUNK_SIZE).enumerate() {
            let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
            let is_last = i + 1 == total_chunks;
            let m = if is_last { 0 } else { 1 };

            if in_tmux {
                out.push_str("\x1bPtmux;\x1b");
            }

            if i == 0 {
                out.push_str(&format!(
                    "\x1b_Ga=T,f=100,t=d,c={width},r={height},C=1,m={m};{chunk_str}\x1b\\"
                ));
            } else {
                out.push_str(&format!("\x1b_Gm={m};{chunk_str}\x1b\\"));
            }

            if in_tmux {
                out.push_str("\x1b\\");
            }
        }
    } else {
        // File transmission (t=f): payload is base64 of absolute filesystem path
        let path_str = abs_path.to_string_lossy();
        let b64_path = base64_encode(path_str.as_bytes());

        if in_tmux {
            out.push_str("\x1bPtmux;\x1b");
        }

        out.push_str(&format!(
            "\x1b_Ga=T,f=100,t=f,c={width},r={height},C=1;{b64_path}\x1b\\"
        ));

        if in_tmux {
            out.push_str("\x1b\\");
        }
    }

    Ok(out)
}

/// Try displaying using `kitten icat` if explicitly requested or available.
pub fn run_kitten_icat(
    path: &Path,
    width: usize,
    height: usize,
    pad_left: usize,
    pad_top: usize,
) -> Result<String, String> {
    let place = format!("{width}x{height}@{pad_left}x{pad_top}");
    let output = Command::new("kitten")
        .args([
            "icat",
            "-n",
            "--stdin=no",
            "--align=left",
            &format!("--place={place}"),
            "--scale-up",
            &path.to_string_lossy(),
        ])
        .output()
        .or_else(|_| {
            Command::new("kitty")
                .args([
                    "+kitten",
                    "icat",
                    "-n",
                    "--stdin=no",
                    "--align=left",
                    &format!("--place={place}"),
                    "--scale-up",
                    &path.to_string_lossy(),
                ])
                .output()
        })
        .map_err(|e| format!("failed to execute kitten icat: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Options for Kitty image rendering.
#[derive(Debug, Clone, Copy, Default)]
pub struct KittyImageOptions {
    pub req_w: Option<usize>,
    pub req_h: Option<usize>,
    pub padding_top: usize,
    pub padding_left: usize,
    pub padding_right: usize,
    pub direct: bool,
    pub icat: bool,
}

/// Render the final output lines for Kitty graphics logo alongside module lines.
pub fn render_kitty_image_lines(
    path: &Path,
    opts: &KittyImageOptions,
    module_lines: &[String],
) -> Result<Vec<String>, String> {
    let px_dims = probe_image_dimensions(path);
    let (width, height) = calculate_cell_dimensions(px_dims, opts.req_w, opts.req_h);

    let kitty_escape = if opts.icat {
        run_kitten_icat(path, width, height, opts.padding_left, opts.padding_top)
            .or_else(|_| generate_kitty_escape(path, width, height, opts.direct))?
    } else {
        generate_kitty_escape(path, width, height, opts.direct)?
    };

    let gap = opts.padding_right.max(3);
    let offset = opts.padding_left + width + gap;

    let total_lines = (opts.padding_top + height).max(module_lines.len());
    let mut out = Vec::with_capacity(total_lines + opts.padding_top + 1);

    // Initial image emission with C=1 (cursor remains at start of image)
    let mut header = String::new();
    if opts.padding_top > 0 {
        header.push_str(&"\n".repeat(opts.padding_top));
    }
    if opts.padding_left > 0 {
        header.push_str(&format!("\x1b[{}C", opts.padding_left));
    }
    header.push_str(&kitty_escape);

    // Interleave text lines with cursor forward movements (\x1b[<offset>C)
    for i in 0..total_lines {
        let text = module_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let line_content = if i == 0 {
            format!("{header}\x1b[{offset}C{text}")
        } else if !text.is_empty() {
            format!("\x1b[{offset}C{text}")
        } else {
            format!("\x1b[{offset}C")
        };
        out.push(line_content);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_png_probe_dimensions() {
        let mut png = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        // 4 bytes length, 4 bytes IHDR
        png.extend_from_slice(&[0, 0, 0, 13]);
        png.extend_from_slice(b"IHDR");
        // width 640 (0x00000280), height 480 (0x000001e0)
        png.extend_from_slice(&640u32.to_be_bytes());
        png.extend_from_slice(&480u32.to_be_bytes());
        let dims = probe_dimensions_from_bytes(&png);
        assert_eq!(dims, Some((640, 480)));
    }

    #[test]
    fn test_calculate_cell_dimensions() {
        // 640x480 pixel image -> aspect (640/480)*2 = 2.666
        let (w, h) = calculate_cell_dimensions(Some((640, 480)), None, None);
        assert!((20..=45).contains(&w));
        assert!((8..=20).contains(&h));

        // Requested width 30
        let (w, h) = calculate_cell_dimensions(Some((640, 480)), Some(30), None);
        assert_eq!(w, 30);
        assert_eq!(h, 11);

        // Explicit width and height
        let (w, h) = calculate_cell_dimensions(None, Some(40), Some(20));
        assert_eq!(w, 40);
        assert_eq!(h, 20);
    }
}

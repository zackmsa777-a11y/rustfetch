use std::env;
use std::ffi::CStr;
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Options {
    pub no_color: bool,
    pub no_logo: bool,
    pub help: bool,
    pub version: bool,
}

pub fn options(args: &[String]) -> Result<Options, String> {
    let mut opts = Options::default();
    for arg in args {
        match arg.as_str() {
            "--no-color" => opts.no_color = true,
            "--no-logo" => opts.no_logo = true,
            "-h" | "--help" => opts.help = true,
            "-v" | "--version" => opts.version = true,
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }
    Ok(opts)
}

pub fn value<'a>(input: &'a str, key: &str, delimiter: char) -> Option<&'a str> {
    for line in input.lines() {
        if let Some((k, v)) = line.split_once(delimiter) {
            if k.trim() != key {
                continue;
            }
            let v = v.trim();
            let v = if (v.starts_with('"') && v.ends_with('"') && v.len() >= 2)
                || (v.starts_with('\'') && v.ends_with('\'') && v.len() >= 2)
            {
                &v[1..v.len() - 1]
            } else {
                v
            };
            return Some(v);
        }
    }
    None
}

pub fn clean(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}

pub fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            out.push(c);
        }
    }
    out
}

pub fn visible_width(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

pub fn uptime(input: &str) -> Option<String> {
    let first = input.split_whitespace().next()?;
    let secs: f64 = first.parse().ok()?;
    if secs < 0.0 || !secs.is_finite() {
        return None;
    }
    let total_secs = secs as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 || days > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    parts.push(format!("{} min{}", minutes, if minutes == 1 { "" } else { "s" }));

    Some(parts.join(", "))
}

pub fn memory(input: &str) -> Option<String> {
    let total_str = value(input, "MemTotal", ':')?;
    let avail_str = value(input, "MemAvailable", ':')?;

    let parse_kb = |s: &str| -> Option<u64> {
        s.split_whitespace().next()?.parse::<u64>().ok()
    };

    let total = parse_kb(total_str)?;
    let available = parse_kb(avail_str)?;

    if total == 0 || available > total {
        return None;
    }

    let used = total - available;
    let total_gib = total as f64 / 1048576.0;
    let used_gib = used as f64 / 1048576.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%)"))
}

pub fn swap(input: &str) -> Option<String> {
    let total_str = value(input, "SwapTotal", ':')?;
    let free_str = value(input, "SwapFree", ':')?;

    let parse_kb = |s: &str| -> Option<u64> {
        s.split_whitespace().next()?.parse::<u64>().ok()
    };

    let total = parse_kb(total_str)?;
    let free = parse_kb(free_str)?;

    if total == 0 {
        return Some("Disabled".into());
    }
    if free > total {
        return None;
    }

    let used = total - free;
    let total_gib = total as f64 / 1048576.0;
    let used_gib = used as f64 / 1048576.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%)"))
}

pub fn format_os(os_release: &str, arch: &str) -> Option<String> {
    let name = value(os_release, "NAME", '=');
    let version = value(os_release, "VERSION", '=');
    let pretty_name = value(os_release, "PRETTY_NAME", '=');

    let base = match (name, version, pretty_name) {
        (Some(n), Some(v), _) => {
            if v.starts_with(n) {
                v.to_string()
            } else {
                format!("{n} {v}")
            }
        }
        (_, _, Some(p)) => p.to_string(),
        (Some(n), None, None) => n.to_string(),
        _ => return None,
    };

    let base_clean = clean(&base);
    if base_clean.contains(arch) {
        Some(base_clean)
    } else {
        Some(format!("{base_clean} {arch}"))
    }
}

pub fn format_host(
    vendor: Option<&str>,
    product: Option<&str>,
    version: Option<&str>,
    virt: Option<&str>,
) -> Option<String> {
    let vendor = vendor.map(clean).filter(|s| !s.is_empty() && s != "None" && s != "To be filled by O.E.M.");
    let product = product.map(clean).filter(|s| !s.is_empty() && s != "None" && s != "To be filled by O.E.M.");
    let version = version.map(clean).filter(|s| !s.is_empty() && s != "None" && s != "To be filled by O.E.M.");
    let virt = virt.map(clean).filter(|s| !s.is_empty() && s != "none");

    let mut parts = Vec::new();

    if let Some(ref v) = virt {
        if v.eq_ignore_ascii_case("kvm") || v.eq_ignore_ascii_case("qemu") {
            parts.push("KVM/QEMU".to_string());
        } else if !v.is_empty() {
            parts.push(v.to_uppercase());
        }
    }

    match (vendor, product) {
        (Some(v), Some(p)) => {
            if parts.is_empty() && !p.contains(&v) {
                parts.push(v);
            }
            parts.push(p);
        }
        (None, Some(p)) => {
            parts.push(p);
        }
        (Some(v), None) => {
            parts.push(v);
        }
        (None, None) => {
            if parts.is_empty() {
                return None;
            }
        }
    }

    let mut host_str = parts.join(" ");

    if let Some(ver) = version {
        if !host_str.contains(&ver) {
            host_str.push_str(&format!(" ({ver})"));
        }
    }

    Some(host_str)
}

pub fn format_cpu(cpuinfo: &str) -> Option<String> {
    let model = value(cpuinfo, "model name", ':')?;
    let cleaned_model = clean(model);

    // Count physical/logical cores
    let count = cpuinfo
        .lines()
        .filter(|line| line.starts_with("processor\t") || line.starts_with("processor:"))
        .count();

    // Check MHz
    let mhz = value(cpuinfo, "cpu MHz", ':')
        .and_then(|m| m.parse::<f64>().ok());

    // Clean model name: remove "CPU @" or redundant spaces
    let mut name = cleaned_model;
    if let Some(idx) = name.find(" CPU @") {
        name.truncate(idx);
    } else if let Some(idx) = name.find(" @") {
        name.truncate(idx);
    }
    let name = name.trim().to_string();

    let mut result = name;
    if count > 0 {
        result.push_str(&format!(" ({count})"));
    }
    if let Some(m) = mhz {
        let ghz = m / 1000.0;
        result.push_str(&format!(" @ {ghz:.2} GHz"));
    }

    Some(result)
}

pub fn format_gpu() -> Option<String> {
    // Check PCI devices in /sys/bus/pci/devices
    if let Ok(entries) = fs::read_dir("/sys/bus/pci/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            let class = fs::read_to_string(path.join("class")).unwrap_or_default();
            // PCI display controller class 0x03xxxx
            if class.trim().starts_with("0x03") {
                let vendor = fs::read_to_string(path.join("vendor")).unwrap_or_default();
                let device = fs::read_to_string(path.join("device")).unwrap_or_default();
                let v = vendor.trim().trim_start_matches("0x");
                let d = device.trim().trim_start_matches("0x");

                if v.eq_ignore_ascii_case("1234") && d.eq_ignore_ascii_case("1111") {
                    return Some("QEMU Virtual Video Controller".into());
                } else if v.eq_ignore_ascii_case("1af4") {
                    return Some("Red Hat VirtIO GPU".into());
                } else if v.eq_ignore_ascii_case("15ad") {
                    return Some("VMware SVGA II Adapter".into());
                } else if v.eq_ignore_ascii_case("80ee") {
                    return Some("VirtualBox Graphics Adapter".into());
                }
            }
        }
    }

    // Fallback to lspci if available
    if let Ok(output) = Command::new("lspci").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("VGA compatible controller") || line.contains("3D controller") || line.contains("Display controller") {
                if let Some((_, rest)) = line.split_once(": ") {
                    let cleaned = rest.trim();
                    if cleaned.contains("1234:1111") {
                        return Some("QEMU Virtual Video Controller".into());
                    }
                    return Some(clean(cleaned));
                }
            }
        }
    }

    None
}

pub fn format_disk(mount_path: &str) -> Option<String> {
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let c_path = std::ffi::CString::new(mount_path).ok()?;

    if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
        return None;
    }

    let total = stat.f_blocks as u64 * stat.f_frsize as u64;
    let avail = stat.f_bavail as u64 * stat.f_frsize as u64;
    if total == 0 || avail > total {
        return None;
    }

    let used = total - avail;
    let total_gib = total as f64 / 1073741824.0;
    let used_gib = used as f64 / 1073741824.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    // Find filesystem type from /proc/mounts
    let fs_type = fs::read_to_string("/proc/mounts").ok().and_then(|mounts| {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == mount_path {
                return Some(parts[2].to_string());
            }
        }
        None
    }).unwrap_or_else(|| "ext4".into());

    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%) - {fs_type}"))
}

pub fn format_packages() -> Option<String> {
    let mut counts = Vec::new();

    // dpkg
    if let Ok(content) = fs::read_to_string("/var/lib/dpkg/status") {
        let count = content.lines().filter(|l| l.starts_with("Status: install ok installed")).count();
        if count > 0 {
            counts.push(format!("{count} (dpkg)"));
        }
    }

    // nix
    if Path::new("/nix/var/nix/profiles/default").exists() {
        if let Ok(output) = Command::new("nix-store")
            .args(["--query", "--requisites", "/nix/var/nix/profiles/default"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            let count = text.lines()
                .filter(|l| {
                    if !Path::new(l).is_dir() {
                        return false;
                    }
                    let base = Path::new(l).file_name().and_then(|f| f.to_str()).unwrap_or("");
                    if base.starts_with("nixos-system-nixos-")
                        || base.ends_with("-doc")
                        || base.ends_with("-man")
                        || base.ends_with("-info")
                        || base.ends_with("-dev")
                        || base.ends_with("-bin")
                    {
                        return false;
                    }
                    let bytes = base.as_bytes();
                    for i in 1..bytes.len().saturating_sub(1) {
                        if bytes[i] == b'.' && bytes[i - 1].is_ascii_digit() && bytes[i + 1].is_ascii_digit() {
                            return true;
                        }
                    }
                    false
                })
                .count();
            if count > 0 {
                counts.push(format!("{count} (nix-default)"));
            }
        }
    }

    // snap
    if let Ok(entries) = fs::read_dir("/snap") {
        let count = entries
            .flatten()
            .filter(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                s != "bin" && s != "README" && !s.starts_with('.')
            })
            .count();
        if count > 0 {
            counts.push(format!("{count} (snap)"));
        }
    }

    // pacman
    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries.flatten().filter(|e| e.path().is_dir()).count();
        if count > 0 {
            counts.push(format!("{count} (pacman)"));
        }
    }

    // flatpak
    if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
        let count = entries.flatten().filter(|e| e.path().is_dir()).count();
        if count > 0 {
            counts.push(format!("{count} (flatpak)"));
        }
    }

    if counts.is_empty() {
        None
    } else {
        Some(counts.join(", "))
    }
}

pub fn format_shell() -> Option<String> {
    let shell_path = env::var("SHELL").ok()?;
    let shell_name = Path::new(&shell_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(&shell_path)
        .to_string();

    // Query version
    let version = Command::new(&shell_path)
        .arg("--version")
        .output()
        .ok()
        .and_then(|out| {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for raw_word in stdout.split_whitespace() {
                let clean_word = raw_word.split(&['(', '-', '+', '~', ','][..]).next().unwrap_or(raw_word);
                let trimmed = clean_word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
                let parts: Vec<&str> = trimmed.split('.').collect();
                if parts.len() >= 2
                    && parts[0].chars().all(|c| c.is_ascii_digit())
                    && parts[1].chars().all(|c| c.is_ascii_digit())
                    && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
                {
                    return Some(trimmed.to_string());
                }
            }
            None
        });

    if let Some(ver) = version {
        Some(format!("{shell_name} {ver}"))
    } else {
        Some(shell_name)
    }
}

pub fn format_display() -> Option<String> {
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let modes_file = path.join("modes");
            if modes_file.is_file() {
                if let Ok(content) = fs::read_to_string(modes_file) {
                    if let Some(mode) = content.lines().next() {
                        let connector = entry
                            .file_name()
                            .to_string_lossy()
                            .trim_start_matches("card0-")
                            .trim_start_matches("card1-")
                            .to_string();
                        return Some(format!("Display ({connector}): {mode}"));
                    }
                }
            }
        }
    }
    None
}

pub fn format_terminal() -> Option<String> {
    let mut parts = Vec::new();

    // Detect pts device
    let pts = unsafe {
        let ptr = libc::ttyname(0);
        if !ptr.is_null() {
            Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
        } else {
            None
        }
    };

    if let Some(p) = pts {
        parts.push(p);
    } else if let Ok(term) = env::var("TERM") {
        parts.push(term);
    }

    // Check SSH connection version
    if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_TTY").is_ok() {
        if let Ok(output) = Command::new("ssh").arg("-V").output() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = if stderr.is_empty() {
                String::from_utf8_lossy(&output.stdout)
            } else {
                stderr
            };
            if let Some(ssh_ver) = combined.split_whitespace().next() {
                let ver_str = ssh_ver.trim_start_matches("OpenSSH_");
                if !ver_str.is_empty() {
                    parts.push(ver_str.to_string());
                }
            }
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

pub fn format_local_ip() -> Option<String> {
    // Find default interface from /proc/net/route
    let route = fs::read_to_string("/proc/net/route").ok()?;
    let mut default_iface = None;
    for line in route.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "00000000" {
            default_iface = Some(parts[0].to_string());
            break;
        }
    }

    let iface_name = default_iface.unwrap_or_else(|| "ens3".into());

    // Use getifaddrs to find IP and netmask
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 || ifap.is_null() {
            return None;
        }

        let mut result = None;
        let mut curr = ifap;
        while !curr.is_null() {
            let ifa = &*curr;
            if !ifa.ifa_addr.is_null() && (*ifa.ifa_addr).sa_family == libc::AF_INET as u16 {
                let name = CStr::from_ptr(ifa.ifa_name).to_string_lossy();
                if name == iface_name {
                    let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                    let ip = Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr));

                    let mask_cidr = if !ifa.ifa_netmask.is_null() {
                        let smask = &*(ifa.ifa_netmask as *const libc::sockaddr_in);
                        u32::from_be(smask.sin_addr.s_addr).count_ones()
                    } else {
                        24
                    };

                    result = Some(format!("{ip}/{mask_cidr}"));
                    break;
                }
            }
            curr = (*curr).ifa_next;
        }

        libc::freeifaddrs(ifap);

        result.map(|ip| format!("Local IP ({iface_name}): {ip}"))
    }
}

pub fn format_locale() -> Option<String> {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .ok()
        .map(|s| clean(&s))
}

const UBUNTU_LOGO: &[&str] = &[
    "                             ....",
    "              .',:clooo:  .:looooo:.",
    "           .;looooooooc  .oooooooooo'",
    "        .;looooool:,''.  :ooooooooooc",
    "       ;looool;.         'oooooooooo,",
    "      ;clool'             .cooooooc.  ,,",
    "         ...                ......  .:oo,",
    "  .;clol:,.                        .loooo'",
    " :ooooooooo,                        'ooool",
    "'ooooooooooo.                        loooo.",
    "'ooooooooool                         coooo.",
    " ,loooooooc.                        .loooo.",
    "   .,;;;'.                          ;ooooc",
    "       ...                         ,ooool.",
    "    .cooooc.              ..',,'.  .cooo.",
    "      ;ooooo:.           ;oooooooc.  :l.",
    "       .coooooc,..      coooooooooo.",
    "         .:ooooooolc:. .ooooooooooo'",
    "           .':loooooo;  ,oooooooooc",
    "               ..';::c'  .;loooo:'",
];

const DEBIAN_LOGO: &[&str] = &[
    "       _,met$$$$$gg.",
    "    ,g$$$$$$$$$$$$$$$P.",
    "  ,g$$P\"\"       \"\"\"Y$$.\".",
    " ,$$P'              `$$$.",
    "',$$P       ,ggs.     `$$b:",
    "`d$$'     ,$P\"'   .    $$$",
    " $$P      d$'     ,    $$P",
    " $$:      $$.   -    ,d$$'",
    " $$;      Y$b._   _,d$P'",
    " Y$$.    `.`\"Y$$$$P\"'",
    " `$$b      \"-.__",
    "  `Y$$",
    "   `Y$$.",
    "     `$$b.",
    "       `Y$$b.",
    "          `\"Y$b._",
    "              `\"\"\"\"",
];

const ARCH_LOGO: &[&str] = &[
    "                  -`",
    "                 .o+`",
    "                `ooo/",
    "               `+oooo:",
    "              `+oooooo:",
    "              -+oooooo+:",
    "            `/:-:++oooo+:",
    "           `/++++/+++++++:",
    "          `/++++++++++++++:",
    "         `/+++ooooooooooooo/`",
    "        ./ooosssso++osssssso+`",
    "       .oossssso-````/ossssss+`",
    "      -osssssso.      :ssssssso.",
    "     :osssssss/        osssso+++.",
    "    /ossssssss/        +ssssooo/-",
    "  `/ossssso+/:-        -:/+osssso+-",
    " `+sso+:-`                 `.-/+oso:",
    "`++:.                           `-/+/",
    ".`                                 `/",
];

const FEDORA_LOGO: &[&str] = &[
    "          /:-------------:\\",
    "       :-------------------::",
    "     :-----------/shhOHbmp---:\\",
    "   /-----------omMMMNNNMMD  ---:",
    "  :-----------sMMMMNMNMP.    ---:",
    " :-----------:MMMdP-------    ---\\ ",
    ",------------:MMh------------  ---:",
    ":------------:FM8HMbar----------  --:",
    ":------------:Mb`:\\_Yblo/-------- --:",
    ":------------:Z`---.:mh/---------- --:",
    ":------/h/---::----:sh-------------:",
    ":-----odN--/hN---omNM------------:",
    ":----:yDN--hNM---omMM-----------:",
    " :---:yM---dMM---omMM-----------:",
    "  :--:m---/NMM---omMM----------:",
    "   \\-:---/hNMM---omMM---------:",
    "     :--/mMMMM---omMM--------:",
    "       :oMMMMMM---omMN-------:",
    "          \\:::::---:::::/",
];

const LINUX_LOGO: &[&str] = &[
    "   .--.",
    "  |o_o |",
    "  |:_/ |",
    " //   \\ \\",
    "(|     | )",
    "/'\\_   _/`\\",
    "\\___)=(___/",
];

fn get_distro_art(distro_id: &str, no_color: bool) -> (Vec<String>, &'static str) {
    let r = if no_color { "" } else { "\x1b[0m" };

    match distro_id.to_lowercase().as_str() {
        "ubuntu" => {
            let c = if no_color { "" } else { "\x1b[1;31m" };
            let key_col = "\x1b[1;31m";
            let lines = UBUNTU_LOGO
                .iter()
                .map(|line| format!("{c}{line}{r}"))
                .collect();
            (lines, key_col)
        }
        "debian" => {
            let c = if no_color { "" } else { "\x1b[1;31m" };
            let key_col = "\x1b[1;31m";
            let lines = DEBIAN_LOGO
                .iter()
                .map(|line| format!("{c}{line}{r}"))
                .collect();
            (lines, key_col)
        }
        "arch" | "archarm" => {
            let c = if no_color { "" } else { "\x1b[1;36m" };
            let key_col = "\x1b[1;36m";
            let lines = ARCH_LOGO
                .iter()
                .map(|line| format!("{c}{line}{r}"))
                .collect();
            (lines, key_col)
        }
        "fedora" => {
            let c = if no_color { "" } else { "\x1b[1;34m" };
            let key_col = "\x1b[1;34m";
            let lines = FEDORA_LOGO
                .iter()
                .map(|line| format!("{c}{line}{r}"))
                .collect();
            (lines, key_col)
        }
        _ => {
            let c = if no_color { "" } else { "\x1b[1;33m" };
            let key_col = "\x1b[1;33m";
            let lines = LINUX_LOGO
                .iter()
                .map(|line| format!("{c}{line}{r}"))
                .collect();
            (lines, key_col)
        }
    }
}

pub struct SystemInfo {
    pub user: String,
    pub hostname: String,
    pub os: Option<String>,
    pub host: Option<String>,
    pub kernel: Option<String>,
    pub uptime: Option<String>,
    pub packages: Option<String>,
    pub shell: Option<String>,
    pub display: Option<String>,
    pub terminal: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<String>,
    pub memory: Option<String>,
    pub swap: Option<String>,
    pub disk: Option<String>,
    pub local_ip: Option<String>,
    pub locale: Option<String>,
    pub distro_id: String,
}

pub fn gather_info() -> SystemInfo {
    let user = clean(
        &env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| "user".into()),
    );

    let hostname = clean(
        fs::read_to_string("/etc/hostname")
            .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname"))
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| "localhost".into())
            .trim(),
    );

    let os_release = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();

    let distro_id = value(&os_release, "ID", '=')
        .unwrap_or("linux")
        .to_string();

    let arch = env::consts::ARCH;
    let os = format_os(&os_release, arch);

    let vendor = fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor")
        .or_else(|_| fs::read_to_string("/sys/class/dmi/id/sys_vendor"))
        .ok();
    let product = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
        .or_else(|_| fs::read_to_string("/sys/class/dmi/id/product_name"))
        .ok();
    let version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
        .or_else(|_| fs::read_to_string("/sys/class/dmi/id/product_version"))
        .ok();
    let virt = Command::new("systemd-detect-virt")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

    let host = format_host(
        vendor.as_deref().map(str::trim),
        product.as_deref().map(str::trim),
        version.as_deref().map(str::trim),
        virt.as_deref(),
    );

    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease")
        .ok()
        .map(|s| format!("Linux {}", clean(s.trim())))
        .filter(|s| !s.is_empty());

    let uptime_val = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| uptime(&s));

    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let memory_val = memory(&meminfo);
    let swap_val = swap(&meminfo);

    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let cpu_val = format_cpu(&cpuinfo);
    let gpu_val = format_gpu();

    let packages_val = format_packages();
    let shell_val = format_shell();
    let display_val = format_display();
    let terminal_val = format_terminal();
    let disk_val = format_disk("/");
    let local_ip_val = format_local_ip();
    let locale_val = format_locale();

    SystemInfo {
        user,
        hostname,
        os,
        host,
        kernel,
        uptime: uptime_val,
        packages: packages_val,
        shell: shell_val,
        display: display_val,
        terminal: terminal_val,
        cpu: cpu_val,
        gpu: gpu_val,
        memory: memory_val,
        swap: swap_val,
        disk: disk_val,
        local_ip: local_ip_val,
        locale: locale_val,
        distro_id,
    }
}

pub fn format_info_lines(info: &SystemInfo, no_color: bool, key_color: &str) -> Vec<String> {
    let reset = if no_color { "" } else { "\x1b[0m" };
    let key_col = if no_color { "" } else { key_color };
    let title_col = if no_color { "" } else { key_color };

    let mut lines = Vec::new();

    let title_plain = format!("{}@{}", info.user, info.hostname);
    let title_display = if no_color {
        title_plain.clone()
    } else {
        format!("{title_col}{}{reset}@{title_col}{}{reset}", info.user, info.hostname)
    };
    let separator = "-".repeat(title_plain.len());

    lines.push(title_display);
    lines.push(separator);

    macro_rules! add_item {
        ($key:expr, $val:expr) => {
            if let Some(v) = $val {
                lines.push(format!("{key_col}{}:{reset} {v}", $key));
            }
        };
    }

    add_item!("OS", info.os.as_ref());
    add_item!("Host", info.host.as_ref());
    add_item!("Kernel", info.kernel.as_ref());
    add_item!("Uptime", info.uptime.as_ref());
    add_item!("Packages", info.packages.as_ref());
    add_item!("Shell", info.shell.as_ref());
    if let Some(ref disp) = info.display {
        if let Some((k, v)) = disp.split_once(':') {
            lines.push(format!("{key_col}{}:{reset}{v}", k.trim()));
        } else {
            lines.push(format!("{key_col}{disp}{reset}"));
        }
    }
    add_item!("Terminal", info.terminal.as_ref());
    add_item!("CPU", info.cpu.as_ref());
    add_item!("GPU", info.gpu.as_ref());
    add_item!("Memory", info.memory.as_ref());
    add_item!("Swap", info.swap.as_ref());
    add_item!("Disk (/)", info.disk.as_ref());
    if let Some(ref ip) = info.local_ip {
        if let Some((k, v)) = ip.split_once(':') {
            lines.push(format!("{key_col}{}:{reset}{v}", k.trim()));
        } else {
            lines.push(format!("{key_col}{ip}{reset}"));
        }
    }
    add_item!("Locale", info.locale.as_ref());

    if !no_color {
        lines.push(String::new());
        // Fastfetch / Neofetch standard 8-color blocks + bright blocks
        lines.push(format!(
            "\x1b[40m   \x1b[41m   \x1b[42m   \x1b[43m   \x1b[44m   \x1b[45m   \x1b[46m   \x1b[47m   {reset}"
        ));
        lines.push(format!(
            "\x1b[100m   \x1b[101m   \x1b[102m   \x1b[103m   \x1b[104m   \x1b[105m   \x1b[106m   \x1b[107m   {reset}"
        ));
    }

    lines
}

pub fn print_fetch(options: &Options) {
    let info = gather_info();
    let (logo_lines, key_color) = get_distro_art(&info.distro_id, options.no_color);
    let info_lines = format_info_lines(&info, options.no_color, key_color);

    if options.no_logo {
        for line in info_lines {
            println!("{line}");
        }
    } else {
        let max_logo_width = logo_lines
            .iter()
            .map(|l| visible_width(l))
            .max()
            .unwrap_or(0);

        let total_lines = logo_lines.len().max(info_lines.len());
        for i in 0..total_lines {
            let logo_part = if i < logo_lines.len() {
                let l = &logo_lines[i];
                let pad = max_logo_width.saturating_sub(visible_width(l));
                format!("{l}{}", " ".repeat(pad))
            } else {
                " ".repeat(max_logo_width)
            };

            if i < info_lines.len() {
                println!("{logo_part}   {}", info_lines[i]);
            } else {
                println!("{logo_part}");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let opts = match options(&args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    if opts.help {
        println!("rustfetch 0.1.0");
        println!("A fast, lightweight system information fetch tool written in Rust.\n");
        println!("USAGE:");
        println!("    rustfetch [OPTIONS]\n");
        println!("OPTIONS:");
        println!("    --no-color     Disable colored output");
        println!("    --no-logo      Do not display ASCII logo");
        println!("    -v, --version  Print version information");
        println!("    -h, --help     Print help information");
        return;
    }

    if opts.version {
        println!("rustfetch 0.1.0");
        return;
    }

    print_fetch(&opts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_exact_keys_and_quoted_values() {
        let input = "NAME=Linux\nPRETTY_NAME=\"Example Linux 1.0\"\n";
        assert_eq!(value(input, "PRETTY_NAME", '='), Some("Example Linux 1.0"));
        assert_eq!(value(input, "MISSING", '='), None);
    }

    #[test]
    fn formats_uptime() {
        assert_eq!(uptime("90061.25 100.0"), Some("1 day, 1 hour, 1 min".into()));
        assert_eq!(uptime("172800.0 0"), Some("2 days, 0 hours, 0 mins".into()));
        assert_eq!(uptime("59.0 0"), Some("0 mins".into()));
        assert_eq!(uptime("invalid"), None);
    }

    #[test]
    fn reports_used_memory_from_available() {
        let input = "MemTotal: 2097152 kB\nMemAvailable: 524288 kB\n";
        assert_eq!(memory(input), Some("1.50 GiB / 2.00 GiB (75%)".into()));
        assert_eq!(memory("MemTotal: 0 kB\nMemAvailable: 0 kB"), None);
        assert_eq!(memory(""), None);
    }

    #[test]
    fn reports_swap_status() {
        let disabled = "SwapTotal: 0 kB\nSwapFree: 0 kB\n";
        assert_eq!(swap(disabled), Some("Disabled".into()));

        let enabled = "SwapTotal: 2097152 kB\nSwapFree: 1048576 kB\n";
        assert_eq!(swap(enabled), Some("1.00 GiB / 2.00 GiB (50%)".into()));
    }

    #[test]
    fn prevents_memory_underflow() {
        assert_eq!(memory("MemTotal: 1024 kB\nMemAvailable: 2048 kB"), None);
    }

    #[test]
    fn formats_os_string() {
        let os_release = "NAME=\"Ubuntu\"\nVERSION=\"24.04.4 LTS (Noble Numbat)\"\n";
        assert_eq!(
            format_os(os_release, "x86_64"),
            Some("Ubuntu 24.04.4 LTS (Noble Numbat) x86_64".into())
        );
    }

    #[test]
    fn formats_host_string() {
        let host = format_host(
            Some("QEMU"),
            Some("Standard PC (i440FX + PIIX, 1996)"),
            Some("pc-i440fx-7.2"),
            Some("kvm"),
        );
        assert_eq!(
            host,
            Some("KVM/QEMU Standard PC (i440FX + PIIX, 1996) (pc-i440fx-7.2)".into())
        );
    }

    #[test]
    fn formats_cpu_string() {
        let cpuinfo = "processor\t: 0\nmodel name\t: Intel(R) Xeon(R) CPU @ 2.60GHz\ncpu MHz\t: 2600.0\nprocessor\t: 1\n";
        assert_eq!(
            format_cpu(cpuinfo),
            Some("Intel(R) Xeon(R) (2) @ 2.60 GHz".into())
        );
    }

    #[test]
    fn sanitizes_terminal_control_characters() {
        assert_eq!(clean("hello\n\u{1b}[31mworld"), "hello[31mworld");
    }

    #[test]
    fn parses_options_and_rejects_unknown_arguments() {
        let options = options(&["--no-color".into(), "--no-logo".into()]).unwrap();
        assert!(options.no_color && options.no_logo);
        assert!(options_for_test("--unknown").is_err());
    }

    fn options_for_test(arg: &str) -> Result<Options, String> {
        options(&[arg.into()])
    }
}

/* Copyright (C) 2026  limforge <limforge@neutronen.net>, justpav05

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License. */

use std::collections::HashMap;
use std::ffi::CStr;
use std::mem;
use std::path::Path;
use std::process;

fn read_file(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn os_release_field(content: &str, key: &str) -> Option<String> {
    content
        .lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.splitn(2, '=').nth(1))
        .map(|v| v.trim_matches('"').to_string())
}

pub struct OsInfo {
    pub display: String,
    pub id: String,
}

pub fn get_os_info() -> OsInfo {
    if cfg!(target_os = "linux") {
        let content = read_file("/etc/os-release")
            .or_else(|| read_file("/usr/lib/os-release"))
            .unwrap_or_default();
        let name = os_release_field(&content, "NAME").unwrap_or_else(|| "Linux".to_string());
        let version = os_release_field(&content, "VERSION_ID").unwrap_or_default();
        let id = os_release_field(&content, "ID")
            .unwrap_or_default()
            .to_ascii_lowercase();
        let display = if version.is_empty() {
            name
        } else {
            format!("{} {}", name, version)
        };
        OsInfo { display, id }
    } else if cfg!(target_os = "freebsd") {
        let version = read_file("/etc/version")
            .or_else(|| {
                process::Command::new("uname")
                    .arg("-r")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string());
        OsInfo {
            display: format!("FreeBSD {}", version.trim()),
            id: "freebsd".to_string(),
        }
    } else {
        OsInfo {
            display: "Unknown".to_string(),
            id: String::new(),
        }
    }
}

pub fn get_kernel() -> String {
    if cfg!(target_os = "freebsd") {
        let ver = process::Command::new("uname")
            .arg("-r")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        format!("kFreeBSD {}", ver)
    } else {
        read_file("/proc/version")
            .and_then(|s| s.split_whitespace().nth(2).map(str::to_string))
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

pub fn get_hostname() -> String {
    let mut buf = [0i8; 256];
    unsafe {
        if libc::gethostname(buf.as_mut_ptr(), buf.len()) == 0 {
            let host = CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned();
            if !host.is_empty() {
                return host;
            }
        }
    }
    read_file("/proc/sys/kernel/hostname")
        .or_else(|| read_file("/etc/hostname"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub struct MemInfo {
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

pub fn get_meminfo() -> MemInfo {
    let content = read_file("/proc/meminfo").unwrap_or_default();
    let mut mem_total = 0u64;
    let mut mem_avail = 0u64;
    let mut swap_total = 0u64;
    let mut swap_free = 0u64;
    let mut done = 0u8;

    for line in content.lines() {
        let Some((key, val)) = line.split_once(':') else {
            continue;
        };
        let kb: u64 = val
            .split_whitespace()
            .next()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        match key {
            "MemTotal" => {
                mem_total = kb;
                done += 1;
            }
            "MemAvailable" => {
                mem_avail = kb;
                done += 1;
            }
            "SwapTotal" => {
                swap_total = kb;
                done += 1;
            }
            "SwapFree" => {
                swap_free = kb;
                done += 1;
            }
            _ => {}
        }
        if done == 4 {
            break;
        }
    }

    MemInfo {
        total: mem_total * 1024,
        used: mem_total.saturating_sub(mem_avail) * 1024,
        swap_total: swap_total * 1024,
        swap_used: swap_total.saturating_sub(swap_free) * 1024,
    }
}

pub fn get_cpu() -> (String, usize) {
    let content = read_file("/proc/cpuinfo").unwrap_or_default();
    let mut brand = None::<String>;
    let mut cores = 0usize;

    for line in content.lines() {
        if brand.is_none() && line.starts_with("model name") {
            brand = line
                .splitn(2, ':')
                .nth(1)
                .map(|s| s.trim().to_string());
        } else if line.starts_with("processor") {
            cores += 1;
        }
        if brand.is_some() && cores > 0 {
            break;
        }
    }

    (
        brand.unwrap_or_else(|| "Unknown CPU".to_string()),
        cores.max(1),
    )
}

fn mount_skip(mount: &str) -> bool {
    mount.starts_with("/proc")
        || mount.starts_with("/sys")
        || mount.starts_with("/dev")
        || mount.starts_with("/run")
}

pub fn get_disks() -> Vec<(String, u64, u64)> {
    let content = read_file("/proc/mounts").unwrap_or_default();
    let mut disk_map: HashMap<String, (u64, u64)> = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let dev = parts.next().unwrap_or("");
        let mount = parts.next().unwrap_or("");
        if !dev.starts_with("/dev/") || mount_skip(mount) {
            continue;
        }

        let base = dev
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .trim_end_matches('p');

        let Ok(path) = std::ffi::CString::new(mount) else {
            continue;
        };
        let mut st: libc::statvfs = unsafe { mem::zeroed() };
        if unsafe { libc::statvfs(path.as_ptr(), &mut st) } != 0 {
            continue;
        }
        let bsize = st.f_frsize as u64;
        let total = st.f_blocks as u64 * bsize;
        let avail = st.f_bavail as u64 * bsize;

        let entry = disk_map.entry(base.to_string()).or_insert((0, 0));
        entry.0 = entry.0.max(total);
        entry.1 = entry.1.max(avail);
    }

    let mut names: Vec<String> = disk_map.keys().cloned().collect();
    names.sort();
    names
        .into_iter()
        .filter_map(|n| {
            let (t, a) = disk_map[&n];
            if t > 0 { Some((n, t, a)) } else { None }
        })
        .collect()
}

pub fn get_init() -> String {
    read_file("/proc/1/comm")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn count_pacman_dir() -> Option<usize> {
    let count = std::fs::read_dir("/var/lib/pacman/local")
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() != "ALPM_DB_VERSION")
        .count();
    if count > 0 { Some(count) } else { None }
}

fn count_dpkg() -> Option<usize> {
    let content = read_file("/var/lib/dpkg/status")?;
    let count = content
        .lines()
        .filter(|l| l.starts_with("Package: "))
        .count();
    if count > 0 { Some(count) } else { None }
}

fn count_xbps() -> Option<usize> {
    if !Path::new("/var/db/xbps").is_dir() {
        return None;
    }
    let count = std::fs::read_dir("/var/db/xbps")
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "plist"))
        .count();
    if count > 0 { Some(count) } else { None }
}

fn count_apk() -> Option<usize> {
    let content = read_file("/lib/apk/db/installed")?;
    let count = content.lines().filter(|l| l.starts_with("P:")).count();
    if count > 0 { Some(count) } else { None }
}

fn count_flatpak() -> Option<usize> {
    if !Path::new("/var/lib/flatpak/app").is_dir() {
        return None;
    }
    let count = std::fs::read_dir("/var/lib/flatpak/app").ok()?.count();
    if count > 0 { Some(count) } else { None }
}

fn count_rpm() -> Option<usize> {
    if !Path::new("/var/lib/rpm").exists() && !Path::new("/usr/lib/sysimage/rpm").exists() {
        return None;
    }
    let output = process::Command::new("rpm")
        .arg("-qa")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let count = output
        .stdout
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .count();
    if count > 0 { Some(count) } else { None }
}

fn count_eopkg() -> Option<usize> {
    if !Path::new("/var/db/eopkg").is_dir() {
        return None;
    }
    let output = process::Command::new("eopkg")
        .args(["list", "--installed", "-q"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let count = output
        .stdout
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .count();
    if count > 0 { Some(count) } else { None }
}

fn count_snap() -> Option<usize> {
    if !Path::new("/snap").is_dir() {
        return None;
    }
    let count = std::fs::read_dir("/snap")
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("current").exists())
        .count();
    if count > 0 { Some(count) } else { None }
}

fn push_pkg(counts: &mut Vec<String>, count: Option<usize>, label: &str) {
    if let Some(n) = count {
        counts.push(format!("{} ({label})", n));
    }
}

pub fn get_packages(distro_id: &str) -> String {
    let mut counts = Vec::new();
    let id = distro_id.to_ascii_lowercase();

    let primary = match id.as_str() {
        "arch" | "cachyos" | "manjaro" | "endeavouros" | "garuda" | "artix" | "arcolinux"
        | "crystal" | "steamos" | "chimera" => Some("pacman"),
        "debian" | "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" | "kali"
        | "parrot" | "devuan" | "mx" | "mxlinux" | "kdeneon" | "neon" | "antix" => {
            Some("dpkg")
        }
        "fedora" | "nobara" | "rhel" | "centos" | "almalinux" | "rocky" | "bazzite"
        | "asahi" | "openeuler" => Some("rpm"),
        "void" => Some("xbps"),
        "alpine" => Some("apk"),
        "solus" => Some("eopkg"),
        _ => None,
    };

    match primary {
        Some("pacman") => push_pkg(&mut counts, count_pacman_dir(), "pacman"),
        Some("dpkg") => push_pkg(&mut counts, count_dpkg(), "dpkg"),
        Some("rpm") => push_pkg(&mut counts, count_rpm(), "rpm"),
        Some("xbps") => push_pkg(&mut counts, count_xbps(), "xbps"),
        Some("apk") => push_pkg(&mut counts, count_apk(), "apk"),
        Some("eopkg") => push_pkg(&mut counts, count_eopkg(), "eopkg"),
        _ => {
            if Path::new("/var/lib/pacman/local").is_dir() {
                push_pkg(&mut counts, count_pacman_dir(), "pacman");
            } else if Path::new("/var/lib/dpkg/status").is_file() {
                push_pkg(&mut counts, count_dpkg(), "dpkg");
            } else if Path::new("/var/lib/rpm").exists() {
                push_pkg(&mut counts, count_rpm(), "rpm");
            } else if Path::new("/var/db/xbps").is_dir() {
                push_pkg(&mut counts, count_xbps(), "xbps");
            } else if Path::new("/lib/apk/db/installed").is_file() {
                push_pkg(&mut counts, count_apk(), "apk");
            }
        }
    }

    push_pkg(&mut counts, count_flatpak(), "flatpak");
    push_pkg(&mut counts, count_snap(), "snap");

    if counts.is_empty() {
        "Unknown".to_string()
    } else {
        counts.join(", ")
    }
}

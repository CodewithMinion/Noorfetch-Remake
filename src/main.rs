/* Copyright (C) 2026  limforge <limforge@neutronen.net>, justpav05


This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::env;
use std::io::{self, IsTerminal, Write};
use std::process;
use std::time::Instant;

#[path = "functions/ascii.rs"]
mod ascii;
use ascii::Distro;
#[path = "functions/config.rs"]
mod config;
#[path = "functions/date.rs"]
mod date;
#[path = "functions/environment.rs"]
mod environment;
#[path = "functions/sysinfo.rs"]
mod sysinfo;

fn color_disabled() -> bool {
    if env::var("NO_COLOR")
        .map(|v| !v.is_empty())
        .unwrap_or(false)
    {
        return true;
    }
    env::var("CLICOLOR").map(|v| v == "0").unwrap_or(false)
}

fn main() {
    let startup = Instant::now();

    let args: Vec<String> = env::args().collect();
    let debug = args.iter().any(|a| a == "--debug" || a == "-d");
    let no_color = args.iter().any(|a| a == "--no-color" || a == "-nc") || color_disabled();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        help_program();
        process::exit(0);
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("noorfetchre {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let isatty = io::stdout().is_terminal();
    let cfg = config::load_config();
    let os_info = sysinfo::get_os_info();
    let os = os_info.display.clone();

    let requested_logo = args
        .iter()
        .find(|a| a.starts_with("--logo="))
        .and_then(|a| a.strip_prefix("--logo=").map(str::to_string));

    let custom_art: Option<String> = cfg
        .custom_art
        .as_deref()
        .filter(|p| !p.is_empty())
        .and_then(|path| {
            if !path.ends_with(".txt") {
                eprintln!(
                    "warning: custom_art '{}' is not a .txt file, ignoring",
                    path
                );
                return None;
            }
            let expanded = if path.starts_with("~/") {
                let home = env::var("HOME").unwrap_or_default();
                format!("{}/{}", home, &path[2..])
            } else {
                path.to_string()
            };
            match std::fs::read_to_string(&expanded) {
                Ok(content) => Some(content),
                Err(e) => {
                    eprintln!(
                        "warning: could not read custom_art '{}': {}, falling back to logo",
                        expanded, e
                    );
                    None
                }
            }
        });

    let art = if let Some(custom) = custom_art {
        custom
    } else {
        let logo_name = requested_logo
            .or_else(|| {
                if cfg.logo != "default" {
                    Some(cfg.logo.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| os.clone());

        let mut distro = Distro::from_string(&logo_name);
        if matches!(distro, Distro::Unknown) && logo_name != "Unknown" {
            eprintln!(
                "warning: logo '{}' not recognized, falling back to auto-detection",
                logo_name
            );
            distro = Distro::from_string(&os);
        }
        distro.ascii_art()
    };

    let use_color = !no_color && isatty;

    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "<unknown>".to_string());

    let hostname = sysinfo::get_hostname();

    let need_mem = cfg.module_enabled("ram") || cfg.module_enabled("swap");
    let mem = need_mem.then(sysinfo::get_meminfo);

    let (cpu_brand, cpu_count) = cfg
        .module_enabled("cpu")
        .then(sysinfo::get_cpu)
        .unwrap_or_else(|| (String::new(), 0));

    let kernel = cfg
        .module_enabled("krnl")
        .then(sysinfo::get_kernel)
        .unwrap_or_default();

    let init = cfg
        .module_enabled("init")
        .then(sysinfo::get_init)
        .unwrap_or_default();

    let days = cfg
        .module_enabled("days")
        .then(date::get_install_days)
        .flatten();

    let packages = cfg
        .module_enabled("packages")
        .then(|| sysinfo::get_packages(&os_info.id))
        .unwrap_or_default();

    let disks = cfg
        .module_enabled("disk")
        .then(sysinfo::get_disks)
        .unwrap_or_default();

    let environment = if cfg.module_enabled("wm") {
        if isatty {
            let wm = environment::get_wm().unwrap_or_else(|| "Unknown".to_string());
            if wm.trim().is_empty() || wm.eq_ignore_ascii_case("tty") || wm == "Unknown" {
                "TTY".to_string()
            } else {
                wm
            }
        } else {
            "TTY".to_string()
        }
    } else {
        String::new()
    };

    let shell = cfg
        .module_enabled("shell")
        .then(|| {
            environment::get_shell()
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .unwrap_or_default();

    let mut entries: Vec<(u32, String, String, (u8, u8, u8))> = Vec::new();

    let add_simple = |entries: &mut Vec<_>,
                      key: &str,
                      fallback_label: &str,
                      value: String,
                      fallback_color: (u8, u8, u8)| {
        if let Some(m) = cfg.modules.get(key) {
            if m.display {
                let label = m.resolve_label(fallback_label).to_string();
                let color = m.resolve_color(fallback_color);
                let value = m.format_value(&[("value", &value)]);
                entries.push((m.order, label, value, color));
            }
        }
    };

    let push_memory_module = |entries: &mut Vec<_>,
                              key: &str,
                              used_bytes: u64,
                              total_bytes: u64,
                              fallback_color: (u8, u8, u8)| {
        if let Some(m) = cfg.modules.get(key) {
            if m.display {
                let used = (used_bytes / 1024 / 1024).to_string();
                let total = (total_bytes / 1024 / 1024).to_string();
                let value = m.format_value(&[
                    ("used", &used),
                    ("total", &total),
                    ("value", &format!("{}/{} MiB", used, total)),
                ]);
                entries.push((
                    m.order,
                    m.resolve_label(key).to_string(),
                    value,
                    m.resolve_color(fallback_color),
                ));
            }
        }
    };

    add_simple(&mut entries, "os", "os", os.clone(), (220, 138, 120));
    add_simple(
        &mut entries,
        "user",
        "user",
        username.clone(),
        (221, 120, 120),
    );
    add_simple(
        &mut entries,
        "hostname",
        "host",
        hostname.clone(),
        (234, 118, 203),
    );
    add_simple(
        &mut entries,
        "shell",
        "shell",
        shell.clone(),
        (32, 159, 181),
    );
    add_simple(
        &mut entries,
        "wm",
        "wm/de",
        environment.clone(),
        (136, 57, 239),
    );

    if let Some(m) = mem {
        push_memory_module(&mut entries, "ram", m.used, m.total, (230, 69, 83));
        if m.swap_used > 0 {
            push_memory_module(
                &mut entries,
                "swap",
                m.swap_used,
                m.swap_total,
                (254, 100, 11),
            );
        }
    }

    if let Some(m) = cfg.modules.get("cpu") {
        if m.display {
            let cores = cpu_count.to_string();
            let value = m.format_value(&[
                ("brand", &cpu_brand),
                ("cores", &cores),
                ("value", &format!("{} ({})", cpu_brand, cores)),
            ]);
            entries.push((
                m.order,
                m.resolve_label("cpu").to_string(),
                value,
                m.resolve_color((223, 142, 29)),
            ));
        }
    }

    if let Some(m) = cfg.modules.get("disk") {
        if m.display {
            let color = m.resolve_color((137, 180, 250));
            let disk_label = m.resolve_label("disk").to_string();

            for (name, total, avail) in &disks {
                let used = total.saturating_sub(*avail);
                let pct = (used as f64 / *total as f64 * 100.0) as u32;

                let bar_total = 10usize;
                let bar_filled = (pct as usize * bar_total / 100).min(bar_total);

                let (br, bg, bb) = if pct < 50 {
                    (166u8, 227u8, 161u8)
                } else if pct < 80 {
                    (249u8, 226u8, 175u8)
                } else {
                    (243u8, 139u8, 168u8)
                };

                let filled_block = if use_color {
                    format!(
                        "\x1b[38;2;{};{};{}m{}\x1b[0m",
                        br,
                        bg,
                        bb,
                        "█".repeat(bar_filled)
                    )
                } else {
                    "█".repeat(bar_filled)
                };

                let bar = format!("[{}{}]", filled_block, " ".repeat(bar_total - bar_filled));
                let used_gb = used / 1024 / 1024 / 1024;
                let total_gb = total / 1024 / 1024 / 1024;
                let value = format!("{} {} {}/{} GB ({}%)", name, bar, used_gb, total_gb, pct);

                entries.push((m.order, disk_label.clone(), value, color));
            }
        }
    }

    add_simple(&mut entries, "krnl", "krnl", kernel.clone(), (64, 160, 43));

    add_simple(&mut entries, "packages", "pkg", packages, (249, 226, 175));

    if let Some(days_val) = days {
        add_simple(&mut entries, "days", "days", days_val, (23, 146, 153));
    }

    add_simple(&mut entries, "init", "init", init.clone(), (4, 165, 229));

    entries.sort_by_key(|(order, ..)| *order);

    let display_rows: Vec<(String, String, (u8, u8, u8))> = entries
        .into_iter()
        .map(|(_, label, value, color)| (label, value, color))
        .collect();

    let mut info_lines: Vec<String> = Vec::new();
    info_lines.push(format!("{}@{}", username, hostname));
    info_lines.push("-".repeat(username.len() + hostname.len() + 1));

    let label_width = display_rows.iter().map(|(l, _, _)| l.len()).max().unwrap_or(6);

    if use_color {
        for (label, value, (r, g, b)) in &display_rows {
            let colored = ansi_bold_color(label, *r, *g, *b);
            let pad = label_width.saturating_sub(visible_len(&colored));
            info_lines.push(format!("{}{} {}", colored, " ".repeat(pad), value));
        }
    } else {
        for (label, value, _) in &display_rows {
            let bold = ansi_bold(label);
            let pad = label_width.saturating_sub(visible_len(&bold));
            info_lines.push(format!("{}{} {}", bold, " ".repeat(pad), value));
        }
    }

    let art_lines: Vec<&str> = art.lines().collect();
    let art_width = art_lines.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let padding = art_width + 5;
    let max_l = art_lines.len().max(info_lines.len());

    let mut out = io::stdout().lock();
    writeln!(out).ok();
    for i in 0..max_l {
        let art_row = art_lines.get(i).copied().unwrap_or("");
        let info_row = info_lines.get(i).map_or("", |s| s.as_str());

        let visible = visible_len(art_row);
        let current_padding = padding.saturating_sub(visible);

        let _ = write!(
            out,
            "{}{:<width$} {}\n",
            art_row,
            "",
            info_row,
            width = current_padding
        );
    }

    if debug {
        let ms = startup.elapsed().as_secs_f64() * 1000.0;
        println!("\nStartup time: {:.2} ms", ms);
    }
}

fn ansi_bold_color(s: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[1;38;2;{};{};{}m{}\x1b[0m", r, g, b, s)
}

fn ansi_bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

fn visible_len(s: &str) -> usize {
    let mut in_ansi = false;
    let mut count = 0;
    let mut prev = '\0';
    for ch in s.chars() {
        if prev == '\x1b' && ch == '[' {
            in_ansi = true;
        } else if in_ansi {
            if ch.is_ascii_alphabetic() {
                in_ansi = false;
            }
        } else if ch != '\x1b' {
            count += if is_wide_char(ch) { 2 } else { 1 };
        }
        prev = ch;
    }
    count
}

fn is_wide_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x1100..=0x115F  |
        0x2E80..=0x303E  |
        0x3041..=0x33BF  |
        0x33FF..=0xA4C6  |
        0xA960..=0xA97C  |
        0xAC00..=0xD7A3  |
        0xF900..=0xFAFF  |
        0xFE10..=0xFE19  |
        0xFE30..=0xFE6B  |
        0xFF01..=0xFF60  |
        0xFFE0..=0xFFE6  |
        0x1B000..=0x1B001|
        0x1F000..=0x1F9FF|
        0x20000..=0x3FFFD
    )
}

fn help_program() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        r#"
Noorfetch Remake — a blazingly fast fetch, written in Rust!

USAGE:
  noorfetchre [OPTION]..

OPTIONS:
  -h,  --help        Display this help menu
  -V,  --version     Print version and exit
  -d,  --debug       Shows the time it took for the program to start
  -nc, --no-color    Disable color for module labels
  --logo=NAME        ASCII logo (e.g. alpine, linuxmint, arch, gentoo)

ENVIRONMENT:
  NO_COLOR           When set (non-empty), disables color output
  CLICOLOR=0         Disables color output

Noorfetch Remake is licensed under GNU LGPL v3.0 or later.
SOURCE:
  https://codeberg.org/limforge/noorfetch

Noorfetch Remake v{}. 2026. limforge."#,
        version
    );
}

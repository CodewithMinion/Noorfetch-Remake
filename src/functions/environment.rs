/* Copyright (C) 2026  limforge <limforge@neutronen.net>, justpav05

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License. */

use std::env;
use std::path::Path;

/// Higher index = lower priority when multiple WMs run.
const KNOWN_WM: &[&str] = &[
    "hyprland",
    "sway",
    "niri",
    "river",
    "i3",
    "bspwm",
    "awesome",
    "dwm",
    "qtile",
    "herbstluftwm",
    "leftwm",
    "openbox",
    "fluxbox",
    "icewm",
    "xfwm4",
    "kwin_wayland",
    "kwin_x11",
    "mutter",
    "marco",
    "compiz",
    "enlightenment",
    "wayfire",
    "labwc",
];

fn wm_priority(name: &str) -> Option<usize> {
    KNOWN_WM.iter().position(|&w| w == name)
}

fn wm_from_proc() -> Option<String> {
    let proc = std::fs::read_dir("/proc").ok()?;
    let mut best: Option<(String, usize)> = None;

    for entry in proc.flatten() {
        let name = entry.file_name();
        let pid = name.to_str()?;
        if !pid.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let comm = std::fs::read_to_string(entry.path().join("comm")).ok()?;
        let comm = comm.trim();
        let Some(prio) = wm_priority(comm) else {
            continue;
        };
        if best.as_ref().is_none_or(|(_, b)| prio < *b) {
            best = Some((comm.to_string(), prio));
        }
    }

    best.map(|(name, _)| name)
}

pub fn get_wm() -> Option<String> {
    let from_env = [
        "XDG_CURRENT_DESKTOP",
        "XDG_SESSION_DESKTOP",
        "DESKTOP_SESSION",
    ]
    .iter()
    .find_map(|var| env::var(var).ok().filter(|v| !v.is_empty()));

    if let Some(wm) = from_env {
        return Some(wm);
    }

    wm_from_proc()
}

pub fn get_shell() -> Option<String> {
    let raw = env::var("SHELL").ok().filter(|v| !v.is_empty())?;
    Path::new(&raw)
        .file_name()
        .and_then(|n| n.to_str())
        .map(str::to_string)
}

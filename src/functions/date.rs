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

use std::fs;
use std::time::SystemTime;

fn metadata_timestamp(path: &str) -> Option<SystemTime> {
    fs::metadata(path)
        .ok()
        .and_then(|m| m.created().or_else(|_| m.modified()).ok())
}

pub fn get_install_days() -> Option<String> {
    let now = SystemTime::now();

    let oldest = ["/", "/etc", "/var", "/usr"]
        .iter()
        .filter_map(|p| metadata_timestamp(p))
        .min()?;

    let days = now.duration_since(oldest).ok()?.as_secs() / 86400;
    if days == 0 {
        return None;
    }
    Some(format!("{} days", days))
}

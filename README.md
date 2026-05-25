# Noorfetch Remake

![Static Badge](https://img.shields.io/badge/release-4.0.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/license-GNU_LGPL--v3.0-green?style=flat)

**Noorfetch Remake (arab. نور)** - is a remake of the little-known [Noorfetch](https://codeberg.org/limforge/noorfetch), which is phasing out support and will no longer be updated.
Noorfetch Remake was created to further develop this project.

![RSFetch screenshot](https://i.postimg.cc/fLxnzvW7/noorfetch-screenshot.png)

## Installation
### Build from source

Install Noorfetch Remake from Github:
```bash
git clone https://github.com/CodewithMinion/Noorfetch-Remake
cd Noorfetch-Remake
```

Compile the project using Cargo
```bash
cargo install --path .
```

Run Noorfetch Remake:
```bash
noorfetchre
```

## Configuration

On first run, a default config is created at `~/.config/noorfetchre/config.json` (Linux and BSD). You can toggle modules, colors, labels, and order there.

## Platform support

Noorfetch Remake is primarily aimed at **Linux** (uses `/proc` for CPU, memory, disks, and init). FreeBSD has partial support for OS and kernel detection.

## License
Noorfetch Remake is distributed under the **GNU Lesser General Public License v3.0 or later**.

This project incorporates the following Rust libraries (crates):
* [serde](https://crates.io/crates/serde) - MIT License or Apache-2.0
* [serde_json](https://crates.io/crates/serde_json) - MIT License or Apache-2.0
* [indexmap](https://crates.io/crates/indexmap) - MIT License or Apache-2.0
* [libc](https://crates.io/crates/libc) - MIT License or Apache-2.0

# Changelog (What was reworked and added by me)

## 3.2.x - 4.0.0

- Project renamed to **Noorfetch Remake**, command in terminal: `noorfetchre`
- Configuration: `~/.config/noorfetchre/config.json`
- The build includes all changes from 3.3.0 and 3.2.x (see below)
- Version 4.0 was released thanks to me; the original author planned to release it.

### Distributions
- Added support for: **Alpine**, **Linux Mint**, **SteamOS**, **Devuan**

### Performance
- Collect data only for **enabled** modules in `config.json`
- Single pass through `/proc` to determine the WM
- pacman packet counting via directory
- `hostname` via the `gethostname` syscall
- Buffered output to stdout

### New Features
- `--version` / `-V`, `NO_COLOR`, `CLICOLOR=0`
- RPM/eopkg counting, improved snap and pacman.

---

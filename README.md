# Noorfetch Remake

![Static Badge](https://img.shields.io/badge/release-4.0.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/license-GNU_LGPL--v3.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/Available_on-Homebrew-yellow) ![Static Badge](https://img.shields.io/badge/Available_on-Cargo-red) ![Static Badge](https://img.shields.io/badge/Available_on-AUR-blue)

**Noorfetch Remake (arab. نور)** is a minimalistic and fast summary of information about your computer, written in Rust!

![RSFetch screenshot](https://i.postimg.cc/fLxnzvW7/noorfetch-screenshot.png)

## Installation

### Versatile. Run the installer
Noorfetch Remake can be installed using a shell script. Run ``installer.sh``

### Arch Linux
#### From AUR
```shell
git clone https://aur.archlinux.org/noorfetch.git
cd noorfetch
makepkg -si
```

### 🍺 Homebrew
```bash
brew tap vfthme/lim
brew install noorfetchre
```

### 🦀 Cargo
```bash
cargo install noorfetchre
```

### Build from source

Install Noorfetch Remake from Codeberg:
```bash
git clone https://codeberg.org/limforge/noorfetch
cd noorfetch
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

# Changelog

## 4.0.0

- Проект переименован в **Noorfetch Remake**, команда в терминале: `noorfetchre`
- Конфиг: `~/.config/noorfetchre/config.json`
- Сборка включает все изменения из 3.3.0 и 3.2.x (см. ниже)

## 3.3.0

### Логотипы
- Цветные ASCII-лого для **Alpine**, **Linux Mint**, **SteamOS**, **Devuan**
- Удалены отдельные лого: Oracle, PikaOS, Ultramarine, Qubes, Omarchy

### Исправления
- `--logo=alpine`, `--logo=linuxmint` и другие короткие имена распознаются надёжно

---

## 3.2.x

### Производительность
- Сбор данных только для **включённых** модулей в `config.json`
- Один проход по `/proc` для определения WM
- Подсчёт пакетов pacman через каталог
- `hostname` через syscall `gethostname`
- Буферизованный вывод в stdout

### Новые возможности
- `--version` / `-V`, `NO_COLOR`, `CLICOLOR=0`
- Подсчёт rpm/eopkg, улучшены snap и pacman

---

## Как обновиться

```bash
cd noorfetch
git pull
cargo build --release
cargo install --path .
```

Проверка:

```bash
./target/release/noorfetchre --version
./target/release/noorfetchre --logo=alpine
./target/release/noorfetchre -d
```

Конфиг: `~/.config/noorfetchre/config.json`

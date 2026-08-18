<div align="center">
<img src="https://capsule-render.vercel.app/api?type=waving&color=0:003ca0,100:003cf0&height=220&section=header&text=Spaced%20Repetition%20Organizer&fontSize=38&fontColor=FFFFFF&fontFamily=Lexend&animation=fadeIn&fontAlignY=38" alt="Quantum banner">

<img src="assets/app-icon.svg" alt="SRO Logo" width="120" height="120">

# Spaced Repetition Organizer

**A fast, native spaced repetition study companion built with Rust and Slint.**

<img src="https://github.com/akimov-256/Spaced-Repetition-Organizer/actions/workflows/ci.yml/badge.svg" alt="Build Status">
<img src="https://img.shields.io/badge/Rust-003CDD?style=flat&logo=rust&logoColor=white">
<img src="https://img.shields.io/badge/UI-Slint-003CDD?style=flat">
<img src="https://img.shields.io/badge/Platform-Windows-003CDD?style=flat">
<img src="https://img.shields.io/badge/License-MIT-003CDD?style=flat">

</div>

<br>

<div align="center">
<img src="https://placehold.co/1000x4/003cdd/003cdd" width="100%" height="4">
</div>

<br>

## About

**SRO (Spaced Repetition Organizer)** helps you plan, schedule, and review study material using spaced repetition principles. Organize topics into lessons, track review intervals, and stay on top of what needs revisiting — all in a lightweight native desktop app with no browser overhead.

<br>

## ✨ Features

- 📚 **Topics & Lessons** — organize study material into structured topics with individual lessons
- 🔁 **Spaced Repetition Scheduling** — automatic interval-based review scheduling
- ⚡ **Native Performance** — instant startup, low memory footprint, no Electron
- 💾 **Local-First** — your data stays on your machine

<br>

## 🖼️ Screenshots

<div align="center">
<img src="assets/preview/main-view.png" alt="Main window" width="60%">
<br><br>
<img src="assets/preview/lessons-view.png" alt="Lesson view" width="60%">
</div>

<br>

## 🛠️ Tech Stack

| Layer       | Technology     |
|-------------|----------------|
| Language    | Rust           |
| UI Toolkit  | [Slint](https://slint.dev/) |
| Platform    | Windows        |

<br>

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Cargo (bundled with Rust)

### Build & Run

```bash
git clone https://github.com/yourusername/SRO.git
cd SRO
cargo run --release
```

### Build for Release

```bash
cargo build --release
```

The compiled binary will be available under `target/release/`.

<br>

## 📁 Project Structure

```
SRO/
├── src/
│   ├── main.rs
│   ├── database_manager.rs
│   └── models.rs
├── ui/
│   ├── app-window.slint
│   ├── models.slint
│   ├── components/
│   ├── dialogs/
│   ├── pages/
│   └── theme/
├── assets/
│   ├── icons/
│   └── app-icon.png
├── Cargo.toml
└── README.md
```

<br>

## 🗺️ Roadmap

- [ ] Import/export decks
- [ ] Statistics dashboard
- [ ] Custom review algorithms
- [ ] Cross-platform builds (macOS, Linux)

<br>

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

<br>

<div align="center">
<img src="https://placehold.co/1000x4/003cdd/003cdd" width="100%" height="4">

<br><br>

Made with 🦀 and 💙

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:003ca0,100:003cf0&height=150&section=footer&animation=fadeIn" alt="Quantum footer banner">

</div>

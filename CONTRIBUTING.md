# Contributing to Spaced Repetition Organizer (SRO)

Thanks for your interest in contributing! This document covers everything you need to get set up, make changes, and submit them.

<br>

## 📋 Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Coding Guidelines](#coding-guidelines)
- [Commit Messages](#commit-messages)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Release Process](#release-process)

<br>

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Cargo (bundled with Rust)
- Windows (primary target platform)

### Setup

```bash
git clone https://github.com/yourusername/SRO.git
cd SRO
cargo build
```

Run the app locally:

```bash
cargo run
```

Run tests:

```bash
cargo test
```

<br>

## Project Structure

```
SRO/
├── src/            # Rust application code
├── ui/             # Slint UI files (.slint) and components
├── assets/         # Icons, images, and other static assets
├── installer/      # Inno Setup script for the Windows installer
└── .github/        # CI workflows
```

If you're working on UI, most of what you need lives in `ui/`. Custom Slint components (e.g. `IconButton`, `TextButton`) live in `ui/components/` — please reuse these instead of falling back to the default `Button` widget where styling consistency matters.

<br>

## Development Workflow

1. Fork the repo and create your branch from `main`:
   ```bash
   git checkout -b feature/short-description
   ```
2. Make your changes.
3. Make sure the project builds and tests pass:
   ```bash
   cargo build
   cargo test
   ```
4. Format your code:
   ```bash
   cargo fmt
   ```
5. Check for lint issues:
   ```bash
   cargo clippy -- -D warnings
   ```
6. Push your branch and open a pull request against `main`.

<br>

## Coding Guidelines

- Follow standard Rust conventions (`cargo fmt` + `cargo clippy` should pass cleanly).
- Keep UI logic in `.slint` files where possible; keep business logic in Rust.
- Prefer small, focused components over large monolithic ones — see `IconButton` as a reference pattern.
- Avoid introducing new dependencies unless there's a clear need — keep the app lightweight.
- Add doc comments (`///`) for public functions and non-obvious logic.

<br>

## Commit Messages

Keep commits focused and messages descriptive. Prefixing with a type is encouraged:

```
feat: add lesson scheduling algorithm
fix: correct icon centering in dialog buttons
docs: update README build instructions
chore: bump dependency versions
```

<br>

## Submitting a Pull Request

- Keep PRs focused on a single change/feature when possible.
- Describe **what** changed and **why** in the PR description.
- Link any related issues (`Closes #12`).
- Make sure CI passes (build + tests run automatically on every PR).
- Be responsive to review feedback — small follow-up commits are fine, no need to force-push unless requested.

<br>

## Reporting Bugs

Open an issue and include:

- Steps to reproduce
- Expected vs actual behavior
- OS/Windows version
- App version (or commit hash if built from source)
- Screenshots, if UI-related

<br>

## Suggesting Features

Open an issue describing:

- The problem you're trying to solve
- Your proposed solution
- Any alternatives you considered

Feature discussions are welcome before implementation — feel free to open an issue to discuss the approach before submitting a large PR.

<br>

## Release Process

Releases are automated via GitHub Actions:

- Every push/PR to `main` triggers a build + test run.
- Pushing a version tag (`v1.2.3`) triggers a release build, packages a Windows installer via Inno Setup, and publishes it as a GitHub Release.

Maintainers only: to cut a release —

```bash
git tag v1.2.3
git push origin v1.2.3
```

<br>

---

By contributing, you agree that your contributions will be licensed under the same license as this project (MIT).

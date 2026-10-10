# PhantomAOS — Development Roadmap

PhantomAOS is a from-scratch, Rust-powered terminal and Linux environment for Android, built for security researchers, ethical hackers, and developers. Works on rooted and non-rooted phones: no root required for core functionality. Memory-safe by design. Authorized-use only.

This roadmap is intentionally sequential: each phase must work and be tested before the next one starts. No feature ships on a promise — it ships when it's running on a real device.

---

## Guiding Principles

- **Security-first**: memory-safe core, sandboxed execution, no telemetry.
- **No root required** for the core app; root unlocks clearly-labeled advanced features only.
- **Authorized use only** — built for ethical, permitted security work.
- **Honesty over hype**: no "5-second install, zero errors" type claims until proven on-device.
- **Desktop-first testing**: anything that can be tested outside Android (like the pty engine) is tested there first — faster iteration, easier debugging.

---

## Phase 0 — Foundation
- [ ] Rust workspace + Android module repo structure finalized
- [x] License chosen: GPL-3.0
- [ ] `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`
- [ ] Authorized-Use Notice finalized (in-app disclaimer + README)
- [ ] CI skeleton: GitHub Actions running `cargo build` + `cargo test` on push

**Exit criteria:** empty-but-structured repo, builds green on CI.

---

## Phase 1 — Rust Terminal Core (desktop-first)
- [ ] Real PTY allocation via `forkpty()` (`nix` crate)
- [ ] Shell process spawn with bidirectional I/O streaming
- [ ] Full job control (Ctrl+C, Ctrl+Z, fg/bg process groups)
- [ ] ANSI/VT100 escape sequence parser
- [ ] CLI test harness: run `vim`, `top`, `ssh` through the engine on Linux desktop

**Exit criteria:** core binary correctly runs full-screen interactive apps — on desktop, before Android is touched at all.

---

## Phase 2 — Android Bridge (JNI)
- [ ] Cross-compile core to `.so` for `aarch64-linux-android` (+ `armv7` fallback)
- [ ] JNI bindings, Rust ⇄ Kotlin
- [ ] Minimal Kotlin + Jetpack Compose shell: one terminal tab, scrollback, keyboard input

**Exit criteria:** same pty engine now runs inside a real Android app — no "tty fd" errors, working job control.

---

## Phase 3 — Phantom Environment (PRoot layer)
- [ ] Rootfs bootstrap (Debian/Ubuntu minimal, arm64)
- [ ] PRoot integration for isolated, per-session Linux environments, no root needed
- [ ] Environment manager UI (create / switch / delete)

**Exit criteria:** user can boot a real Debian shell inside the app and run standard apt packages.

**Known limitation (document honestly, don't hide):** PRoot has overhead vs real namespaces, and some syscalls (raw sockets, certain kernel-dependent tools) won't work without root.

---

## Phase 4 — Package Manager (`paos`)
- [ ] MVP: 5–10 manually compiled, tested tools (nmap, hydra, sqlmap, etc.) hosted on GitHub Releases
- [ ] `paos install <tool>` / `paos upgrade` commands
- [ ] *(Later, not MVP)* GitHub Actions pipeline auto-compiling tracked upstream repos

**Exit criteria:** installing a tool from the curated list is one command, no manual dependency fixing. Scope stays small until proven stable.

---

## Phase 5 — Multi-Tab / Split-Screen UI + Macro Keyboard
- [ ] Multiple terminal tabs
- [ ] 2–3 pane split-screen
- [ ] Customizable macro keyboard (user-defined shortcuts)
- [ ] Theming

---

## Phase 6 — Remote Access + Session Persistence
- [ ] In-app SSH client (key-based auth, saved profiles)
- [ ] In-app VNC client
- [ ] Foreground service + wakelock for long-running sessions, with clear battery-use disclosure

---

## Phase 7 — Encrypted Vault + OPSEC Features
- [ ] AES-256 encrypted local vault (notes, loot, screenshots, credentials)
- [ ] Panic-wipe gesture
- [ ] Root-only features (MAC randomization, monitor-mode injection) clearly labeled as root-dependent, never assumed universal

---

## Phase 8 — Plugin Architecture
- [ ] WASM-based plugin API (sandboxed, capability-based)
- [ ] Sample plugin + contributor documentation

---

## Phase 9 — Public Beta
- [ ] Documentation site
- [ ] Distribution via GitHub Releases + F-Droid (not Play Store — policy conflict with pentest tooling)
- [ ] Issue templates, contribution guide

---

## Non-Goals (for now)
- Root is never required for core functionality
- No "undetectable" or "anti-forensic" claims
- No bundling tools whose license conflicts with GPL-3.0 distribution

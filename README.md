# PhantomAOS

PhantomAOS is a from-scratch, Rust-powered terminal and Linux environment for Android, built for security researchers, ethical hackers, and developers. It works on both rooted and non-rooted phones: no root required for core functionality. Memory-safe by design.

> Status: Early development (Phase 2 - JNI bridge + real PTY engine working). Not yet ready for general use. See [ROADMAP.md](ROADMAP.md) for current progress.

## What makes it different

- **Rust core engine** - real `forkpty()`-based terminal with full job control (Ctrl+C, Ctrl+Z, background processes), not a fake shell wrapper
- **Works with or without root** - isolated Linux environments via PRoot, with clearly-labeled advanced features for rooted phones
- **Memory-safe by design** - a security tool shouldn't itself be an attack surface
- **Built for the job** - split-screen multi-terminal, macro keyboard, SSH/VNC client and a curated package manager (`paos`), all on the [roadmap](ROADMAP.md)

## Documentation

- [ROADMAP.md](ROADMAP.md) - development phases and current progress
- [ARCHITECTURE.md](ARCHITECTURE.md) - technical design and component breakdown
- [Project website](https://phantomaos.github.io/PhantomAOS/)

## Building from source

This project uses a Rust core (`core/phantom-core`) cross-compiled for Android via GitHub Actions, linked into the Android app (`app/`) through JNI. See `.github/workflows/build.yml` for the CI build pipeline.

```
git clone https://github.com/PhantomAOS/PhantomAOS.git
cd PhantomAOS
```

## Community

Join the community on Telegram: [t.me/PhantomAOS_Official](https://t.me/PhantomAOS_Official)

## Authorized Use Only

PhantomAOS provides terminal emulation, network diagnostic, and Linux environment tools designed for developers, systems administrators, and security researchers.

- All security testing, penetration testing, and network tools must only be used on systems and networks you explicitly own or have documented permission to test.
- Unauthorized access, intrusion, or scanning of networks is illegal under applicable computer crime laws.
- The maintainers of this project are not responsible for misuse of these tools.

## License

PhantomAOS is released under the [GPL-3.0 license](LICENSE).


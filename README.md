# Minecraft Server Setup

A polished terminal wizard for creating a reproducible Paper Minecraft server setup. It turns a few plain-language choices into ready-to-run Docker Compose or native Java server files.

## What it does

- Guides you through players, RAM, runtime, Minecraft version, game settings, whitelist, authentication, and PvP.
- Recommends RAM from the player count, while letting you override it.
- Creates portable configuration, `server.properties`, EULA acceptance, and either a `compose.yml` or native setup scripts.
- Docker mode works on Windows, macOS, Linux, and Arch Linux. Native mode creates both PowerShell and POSIX shell scripts.
- Refuses to overwrite a non-empty install folder.

## Quick start

Download a release for your platform, then run it in a terminal:

```text
mcserver-setup
```

For a local build, install the stable [Rust toolchain](https://rustup.rs/) and run:

```sh
cargo run --release
```

After the wizard completes:

| Choice | Start your server |
| --- | --- |
| Docker | `cd minecraft-server && docker compose up -d` |
| Windows native | `cd minecraft-server; .\install-server.ps1` |
| macOS/Linux/Arch native | `cd minecraft-server && sh install-server.sh` |

Native installation requires Java 21+ and internet access. Docker installation requires Docker Desktop (Windows/macOS) or Docker Engine with the Compose plugin (Linux/Arch).

## Linux and Arch prerequisites

For Docker on Arch: `sudo pacman -S docker docker-compose-plugin`, then enable the Docker service and add your user to the `docker` group. For native mode: `sudo pacman -S jre21-openjdk curl`.

The native script uses Paper's official API at install time. By using this project, you agree to Minecraft's EULA; the wizard writes `eula=true` intentionally and visibly.

## Quality gates

Every concern is an independent CI block: formatting, lint/unit tests, a three-OS release build matrix, and an isolated Arch Linux compile/test container. This means the status checks can be required individually in GitHub branch protection.

Run the same checks locally:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release --locked
```

## Publishing releases

CI uploads built binaries as workflow artifacts. To publish binaries as GitHub Releases, add a tag workflow when you're ready; the repository intentionally starts with a conservative CI-only policy.

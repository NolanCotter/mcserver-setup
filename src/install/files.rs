use std::{fs, path::Path};

use anyhow::{bail, Context, Result};

use crate::config::{Runtime, ServerConfig};

pub fn install(config: &ServerConfig) -> Result<()> {
    config.validate().map_err(anyhow::Error::msg)?;
    let dir = &config.directory;
    if dir.exists() && fs::read_dir(dir)?.next().is_some() {
        bail!("Install directory is not empty: {}", dir.display());
    }
    fs::create_dir_all(dir).with_context(|| format!("Could not create {}", dir.display()))?;
    write(dir.join("server.properties"), &server_properties(config))?;
    write(dir.join("eula.txt"), "eula=true\n")?;
    write(dir.join("mcserver.toml"), &toml::to_string_pretty(config)?)?;
    match config.runtime {
        Runtime::Docker => write(dir.join("compose.yml"), &compose(config))?,
        Runtime::Native => {
            write(dir.join("install-server.sh"), &unix_installer(config))?;
            write(dir.join("install-server.ps1"), &windows_installer(config))?;
        }
    }
    Ok(())
}

fn write(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    fs::write(path, contents)?;
    Ok(())
}

fn server_properties(c: &ServerConfig) -> String {
    format!("motd={}\nmax-players={}\ngamemode={}\ndifficulty={}\nonline-mode={}\nwhite-list={}\npvp={}\nserver-port=25565\n", c.name, c.max_players, c.gamemode.label().to_lowercase(), c.difficulty, c.online_mode, c.whitelist, c.pvp)
}

fn compose(c: &ServerConfig) -> String {
    format!("services:\n  minecraft:\n    image: itzg/minecraft-server:java21\n    container_name: minecraft-server\n    ports:\n      - \"25565:25565\"\n    environment:\n      EULA: \"TRUE\"\n      TYPE: PAPER\n      VERSION: \"{}\"\n      MEMORY: \"{}G\"\n      MAX_PLAYERS: \"{}\"\n      MODE: \"{}\"\n      DIFFICULTY: \"{}\"\n      ONLINE_MODE: \"{}\"\n      ENABLE_WHITELIST: \"{}\"\n      PVP: \"{}\"\n      MOTD: \"{}\"\n    volumes:\n      - ./data:/data\n    restart: unless-stopped\n", c.version, c.memory_gb, c.max_players, c.gamemode.label().to_lowercase(), c.difficulty, c.online_mode, c.whitelist, c.pvp, c.name)
}

fn unix_installer(c: &ServerConfig) -> String {
    format!(
        r#"#!/usr/bin/env sh
set -eu
# Works on macOS, Ubuntu/Debian, Fedora, and Arch Linux. Requires Java 21+ and curl.
VERSION="{}"
JAR=paper.jar
if [ "$VERSION" = LATEST ]; then VERSION=$(curl -fsSL https://api.papermc.io/v2/projects/paper | sed -n 's/.*"versions":\[.*"\([^"]*\)"\].*/\1/p'); fi
BUILD=$(curl -fsSL https://api.papermc.io/v2/projects/paper/versions/$VERSION | sed -n 's/.*"builds":\[.*\([0-9][0-9]*\)\].*/\1/p')
curl -fL "https://api.papermc.io/v2/projects/paper/versions/$VERSION/builds/$BUILD/downloads/paper-$VERSION-$BUILD.jar" -o $JAR
java -Xms{}G -Xmx{}G -jar $JAR --nogui
"#,
        c.version, c.memory_gb, c.memory_gb
    )
}

fn windows_installer(c: &ServerConfig) -> String {
    format!("$ErrorActionPreference = 'Stop'\n# Requires Java 21+ and PowerShell 5+.\n$version = '{}'\nif ($version -eq 'LATEST') {{ $version = (Invoke-RestMethod 'https://api.papermc.io/v2/projects/paper').versions[-1] }}\n$build = (Invoke-RestMethod \"https://api.papermc.io/v2/projects/paper/versions/$version\").builds[-1]\nInvoke-WebRequest \"https://api.papermc.io/v2/projects/paper/versions/$version/builds/$build/downloads/paper-$version-$build.jar\" -OutFile paper.jar\njava -Xms{}G -Xmx{}G -jar paper.jar --nogui\n", c.version, c.memory_gb, c.memory_gb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn docker_install_writes_runnable_files() {
        let temp = tempdir().unwrap();
        let c = ServerConfig {
            directory: temp.path().join("server"),
            ..Default::default()
        };
        install(&c).unwrap();
        assert!(c.directory.join("compose.yml").exists());
        assert!(fs::read_to_string(c.directory.join("compose.yml"))
            .unwrap()
            .contains("itzg/minecraft-server"));
    }
    #[test]
    fn native_install_has_both_platform_scripts() {
        let temp = tempdir().unwrap();
        let c = ServerConfig {
            directory: temp.path().join("server"),
            runtime: Runtime::Native,
            ..Default::default()
        };
        install(&c).unwrap();
        assert!(c.directory.join("install-server.sh").exists());
        assert!(c.directory.join("install-server.ps1").exists());
    }
}

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Runtime {
    #[default]
    Docker,
    Native,
}

impl Runtime {
    pub const ALL: [Self; 2] = [Self::Docker, Self::Native];
    pub fn label(self) -> &'static str {
        match self {
            Self::Docker => "Docker Compose",
            Self::Native => "Native Java",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    #[default]
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub const ALL: [Self; 4] = [
        Self::Survival,
        Self::Creative,
        Self::Adventure,
        Self::Spectator,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Survival => "Survival",
            Self::Creative => "Creative",
            Self::Adventure => "Adventure",
            Self::Spectator => "Spectator",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub directory: PathBuf,
    pub max_players: u16,
    pub memory_gb: u8,
    pub version: String,
    pub runtime: Runtime,
    pub gamemode: GameMode,
    pub difficulty: String,
    pub online_mode: bool,
    pub whitelist: bool,
    pub pvp: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "My Minecraft Server".into(),
            directory: PathBuf::from("minecraft-server"),
            max_players: 10,
            memory_gb: 2,
            version: "LATEST".into(),
            runtime: Runtime::Docker,
            gamemode: GameMode::Survival,
            difficulty: "normal".into(),
            online_mode: true,
            whitelist: false,
            pvp: true,
        }
    }
}

impl ServerConfig {
    pub fn recommended_memory(players: u16) -> u8 {
        match players {
            0..=5 => 2,
            6..=15 => 4,
            16..=35 => 6,
            _ => 8,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Server name cannot be empty.".into());
        }
        if self.max_players == 0 {
            return Err("Choose at least one player.".into());
        }
        if self.memory_gb == 0 {
            return Err("Allocate at least 1 GB of memory.".into());
        }
        if self.directory.as_os_str().is_empty() {
            return Err("Choose an install folder.".into());
        }
        Ok(())
    }
}

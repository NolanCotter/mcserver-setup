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
    pub motd: String,
    pub seed: String,
    pub max_players: u16,
    pub memory_gb: u8,
    pub version: String,
    pub runtime: Runtime,
    pub gamemode: GameMode,
    pub difficulty: String,
    pub online_mode: bool,
    pub whitelist: bool,
    pub pvp: bool,
    pub port: u16,
    pub view_distance: u8,
    pub simulation_distance: u8,
    pub hardcore: bool,
    pub allow_flight: bool,
    pub command_blocks: bool,
    pub max_world_size: u32,
    pub spawn_protection: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "My Minecraft Server".into(),
            directory: PathBuf::from("minecraft-server"),
            motd: "A Minecraft Server".into(),
            seed: String::new(),
            max_players: 10,
            memory_gb: 2,
            version: "LATEST".into(),
            runtime: Runtime::Docker,
            gamemode: GameMode::Survival,
            difficulty: "normal".into(),
            online_mode: true,
            whitelist: false,
            pvp: true,
            port: 25565,
            view_distance: 10,
            simulation_distance: 10,
            hardcore: false,
            allow_flight: false,
            command_blocks: false,
            max_world_size: 29_999_984,
            spawn_protection: 16,
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
        if self.port < 1024 {
            return Err("Choose a server port from 1024 to 65535.".into());
        }
        if !(2..=32).contains(&self.view_distance) {
            return Err("View distance must be between 2 and 32 chunks.".into());
        }
        if !(2..=32).contains(&self.simulation_distance) {
            return Err("Simulation distance must be between 2 and 32 chunks.".into());
        }
        Ok(())
    }
}

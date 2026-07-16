use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;

use crate::{
    config::{GameMode, Runtime, ServerConfig},
    install,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Welcome,
    Configure,
    Review,
    Complete,
}

pub struct App {
    pub running: bool,
    pub page: Page,
    pub config: ServerConfig,
    pub selected: usize,
    pub editing: bool,
    pub message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            page: Page::Welcome,
            config: ServerConfig::default(),
            selected: 0,
            editing: false,
            message: String::new(),
        }
    }
}

impl App {
    pub const FIELD_COUNT: usize = 12;
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if !self.editing && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
            self.running = false;
            return Ok(());
        }
        match self.page {
            Page::Welcome => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Char('s')) {
                    self.page = Page::Configure;
                }
            }
            Page::Configure => self.configure(key),
            Page::Review => match key.code {
                KeyCode::Char('b') | KeyCode::Left => self.page = Page::Configure,
                KeyCode::Enter => match install::install(&self.config) {
                    Ok(()) => {
                        self.page = Page::Complete;
                        self.message = "Server files created successfully.".into();
                    }
                    Err(err) => self.message = err.to_string(),
                },
                _ => {}
            },
            Page::Complete => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Char('q')) {
                    self.running = false;
                }
            }
        }
        Ok(())
    }
    fn configure(&mut self, key: KeyEvent) {
        if self.editing {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    self.editing = false;
                    self.message.clear();
                }
                KeyCode::Backspace => match self.selected {
                    0 => {
                        self.config.name.pop();
                    }
                    1 => {
                        let mut text = self.config.directory.display().to_string();
                        text.pop();
                        self.config.directory = PathBuf::from(text);
                    }
                    _ => {}
                },
                KeyCode::Char(character) => match self.selected {
                    0 => self.config.name.push(character),
                    1 => {
                        let mut text = self.config.directory.display().to_string();
                        text.push(character);
                        self.config.directory = PathBuf::from(text);
                    }
                    _ => {}
                },
                _ => {}
            }
            return;
        }
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected = (self.selected + 1) % Self::FIELD_COUNT
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected = (self.selected + Self::FIELD_COUNT - 1) % Self::FIELD_COUNT
            }
            KeyCode::Right | KeyCode::Char('l') => self.adjust(1),
            KeyCode::Left | KeyCode::Char('h') => self.adjust(-1),
            KeyCode::Char('e') if self.selected <= 1 => {
                self.editing = true;
                self.message = "Editing — type, Backspace to delete, Enter to save.".into();
            }
            KeyCode::Enter if self.selected == Self::FIELD_COUNT - 1 => {
                if self.config.validate().is_ok() {
                    self.page = Page::Review;
                    self.message.clear();
                } else {
                    self.message = self.config.validate().unwrap_err();
                }
            }
            _ => {}
        }
    }
    fn adjust(&mut self, delta: i8) {
        match self.selected {
            2 => {
                let next = (self.config.max_players as i16 + delta as i16 * 5).clamp(1, 100);
                self.config.max_players = next as u16;
                self.config.memory_gb = ServerConfig::recommended_memory(self.config.max_players);
            }
            3 => self.config.memory_gb = (self.config.memory_gb as i8 + delta).clamp(1, 32) as u8,
            4 => self.config.runtime = cycle(&Runtime::ALL, self.config.runtime, delta),
            5 => self.config.gamemode = cycle(&GameMode::ALL, self.config.gamemode, delta),
            6 => {
                self.config.difficulty = cycle_str(
                    &["peaceful", "easy", "normal", "hard"],
                    &self.config.difficulty,
                    delta,
                )
            }
            7 => self.config.online_mode = !self.config.online_mode,
            8 => self.config.whitelist = !self.config.whitelist,
            9 => self.config.pvp = !self.config.pvp,
            10 => {
                self.config.version =
                    cycle_str(&["LATEST", "1.21.8", "1.21.7"], &self.config.version, delta)
            }
            _ => {}
        }
    }
}
fn cycle<T: Copy + PartialEq>(values: &[T], current: T, delta: i8) -> T {
    let pos = values.iter().position(|x| *x == current).unwrap_or(0) as i8;
    values[(pos + delta).rem_euclid(values.len() as i8) as usize]
}
fn cycle_str(values: &[&str], current: &str, delta: i8) -> String {
    let pos = values.iter().position(|x| *x == current).unwrap_or(0) as i8;
    values[(pos + delta).rem_euclid(values.len() as i8) as usize].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }
    #[test]
    fn first_run_user_flow_reaches_review() {
        let mut app = App::default();
        app.handle_key(key(KeyCode::Enter)).unwrap();
        assert_eq!(app.page, Page::Configure);
        for _ in 0..App::FIELD_COUNT - 1 {
            app.handle_key(key(KeyCode::Down)).unwrap();
        }
        app.handle_key(key(KeyCode::Enter)).unwrap();
        assert_eq!(app.page, Page::Review);
    }
    #[test]
    fn player_choice_updates_recommended_memory() {
        let mut app = App {
            page: Page::Configure,
            ..Default::default()
        };
        app.selected = 2;
        app.handle_key(key(KeyCode::Right)).unwrap();
        assert_eq!(app.config.max_players, 15);
        assert_eq!(app.config.memory_gb, 4);
    }
    #[test]
    fn name_and_folder_can_be_edited_in_the_wizard() {
        let mut app = App {
            page: Page::Configure,
            ..Default::default()
        };
        app.handle_key(key(KeyCode::Char('e'))).unwrap();
        app.handle_key(key(KeyCode::Char('!'))).unwrap();
        app.handle_key(key(KeyCode::Enter)).unwrap();
        assert_eq!(app.config.name, "My Minecraft Server!");
        app.selected = 1;
        app.handle_key(key(KeyCode::Char('e'))).unwrap();
        app.handle_key(key(KeyCode::Char('2'))).unwrap();
        app.handle_key(key(KeyCode::Enter)).unwrap();
        assert_eq!(app.config.directory, PathBuf::from("minecraft-server2"));
    }
}

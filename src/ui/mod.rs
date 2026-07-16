use crate::app::{App, Page};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

const MOSS: Color = Color::Rgb(118, 185, 0);
const SKY: Color = Color::Rgb(83, 166, 224);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(18, 23, 20))),
        area,
    );
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(2),
        ])
        .split(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "▣  MINECRAFT SERVER SETUP",
                Style::default().fg(MOSS).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "cross-platform installer",
                Style::default().fg(Color::DarkGray),
            ),
        ]))
        .alignment(Alignment::Center),
        chunks[0],
    );
    match app.page {
        Page::Welcome => welcome(frame, chunks[1]),
        Page::Configure => configure(frame, app, chunks[1]),
        Page::Review => review(frame, app, chunks[1]),
        Page::Complete => complete(frame, app, chunks[1]),
    }
    let help = match app.page {
        Page::Welcome => "Enter  start setup     Q  quit",
        Page::Configure => "↑↓ choose  ←→ change  E edit text fields  Enter review  Q quit",
        Page::Review => "Enter  create server     B  back     Q  quit",
        Page::Complete => "Enter / Q  finish",
    };
    frame.render_widget(
        Paragraph::new(help)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center),
        chunks[2],
    );
}
fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(SKY).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(55, 75, 61)))
}
fn welcome(frame: &mut Frame, area: Rect) {
    let text = "Bring a Minecraft server online without memorizing commands.\n\nThis guided setup chooses sensible resources based on player count, writes safe configuration files, and supports Docker on any OS or a native Java install on Windows, macOS, Linux, and Arch.\n\nYou can always review every setting before anything is created.";
    frame.render_widget(
        Paragraph::new(text)
            .block(panel("Ready to build your world?"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered(area, 76, 15),
    );
}
fn configure(frame: &mut Frame, app: &App, area: Rect) {
    let c = &app.config;
    let values = vec![
        c.name.clone(),
        c.directory.display().to_string(),
        c.motd.clone(),
        if c.seed.is_empty() {
            "Random".into()
        } else {
            c.seed.clone()
        },
        format!("{} players", c.max_players),
        format!("{} GB", c.memory_gb),
        c.runtime.label().into(),
        c.gamemode.label().into(),
        c.difficulty.clone(),
        yes_no(c.online_mode),
        yes_no(c.whitelist),
        yes_no(c.pvp),
        c.version.clone(),
        c.port.to_string(),
        format!("{} chunks", c.view_distance),
        format!("{} chunks", c.simulation_distance),
        yes_no(c.hardcore),
        yes_no(c.allow_flight),
        yes_no(c.command_blocks),
        c.max_world_size.to_string(),
        format!("{} blocks", c.spawn_protection),
        "Review & create →".into(),
    ];
    let labels = [
        "Server name",
        "Install folder",
        "Server message (MOTD)",
        "World seed",
        "Expected players",
        "Memory allocation",
        "How to run",
        "Game mode",
        "Difficulty",
        "Online authentication",
        "Whitelist",
        "Player combat",
        "Minecraft version",
        "Server port",
        "View distance",
        "Simulation distance",
        "Hardcore world",
        "Allow flight",
        "Command blocks",
        "Max world size",
        "Spawn protection",
        "Continue",
    ];
    let lines: Vec<Line> = labels
        .iter()
        .zip(values)
        .enumerate()
        .map(|(i, (label, value))| {
            let chosen = i == app.selected;
            Line::from(vec![
                Span::styled(if chosen { "❯ " } else { "  " }, Style::default().fg(MOSS)),
                Span::styled(
                    format!("{label:<24}"),
                    if chosen {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                ),
                Span::styled(
                    value,
                    if chosen {
                        Style::default().fg(MOSS).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SKY)
                    },
                ),
            ])
        })
        .collect();
    let content = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((App::FIELD_COUNT as u16 + 2).min(area.height.saturating_sub(4))),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(centered(
            area,
            88,
            (App::FIELD_COUNT as u16 + 7).min(area.height),
        ));
    frame.render_widget(
        Paragraph::new(lines).block(panel("Server preferences")),
        content[0],
    );
    let tip = if app.selected <= 3 {
        "Press E to edit server name, folder, MOTD, or world seed."
    } else if app.selected == 4 {
        "Memory is automatically suggested from player count; you can override it."
    } else {
        "Use left/right arrows to adjust the selected option."
    };
    frame.render_widget(
        Paragraph::new(tip)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center),
        content[1],
    );
    if !app.message.is_empty() {
        frame.render_widget(
            Paragraph::new(app.message.as_str())
                .style(Style::default().fg(Color::LightRed))
                .alignment(Alignment::Center),
            content[2],
        );
    }
}
fn review(frame: &mut Frame, app: &App, area: Rect) {
    let c = &app.config;
    let run = if c.runtime == crate::config::Runtime::Docker {
        "Docker Compose (recommended)"
    } else {
        "Native Java 21+"
    };
    let seed = if c.seed.is_empty() { "Random" } else { &c.seed };
    let text = format!("{}\n\n{} • {} players • {} GB RAM\n{} • {} difficulty • port {}\n{} chunks view • {} chunks simulation • seed: {}\nHardcore: {}    Flight: {}    Command blocks: {}\nAuthentication: {}    Whitelist: {}    PvP: {}\n\nInstall folder: {}\n\nPress Enter to create the server files. No network or system changes are made by this step.", c.name, run, c.max_players, c.memory_gb, c.gamemode.label(), c.difficulty, c.port, c.view_distance, c.simulation_distance, seed, yes_no(c.hardcore), yes_no(c.allow_flight), yes_no(c.command_blocks), yes_no(c.online_mode), yes_no(c.whitelist), yes_no(c.pvp), c.directory.display());
    frame.render_widget(
        Paragraph::new(text)
            .block(panel("Review your setup"))
            .wrap(Wrap { trim: true }),
        centered(area, 76, 16),
    );
}
fn complete(frame: &mut Frame, app: &App, area: Rect) {
    let run = if app.config.runtime == crate::config::Runtime::Docker {
        "Run: docker compose up -d"
    } else {
        "Run the platform-appropriate install script in that folder."
    };
    let text = format!("✓ {}\n\n{}\n\n{}\n\nYour EULA acceptance, server properties, and reproducible setup configuration are ready. Back up the folder before large changes.", app.message, app.config.directory.display(), run);
    frame.render_widget(
        Paragraph::new(text)
            .block(panel("All set"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered(area, 76, 15),
    );
}
fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(height.min(area.height)),
            Constraint::Fill(1),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(width.min(area.width)),
            Constraint::Fill(1),
        ])
        .split(vertical[1])[1]
}
fn yes_no(value: bool) -> String {
    if value {
        "Yes".into()
    } else {
        "No".into()
    }
}

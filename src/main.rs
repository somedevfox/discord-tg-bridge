// Copyright (C) 2022 Egor Poleshko
// 
// This file is part of Discord-to-Telegram Bridge Bot.
// 
// Discord-to-Telegram Bridge Bot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Discord-to-Telegram Bridge Bot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Discord-to-Telegram Bridge Bot.  If not, see <http://www.gnu.org/licenses/>.
#![feature(async_closure)]
use std::{fs::File, path::Path, io};
use crossterm::{execute, terminal::EnterAlternateScreen, event::{EnableMouseCapture}};
use gettext::Catalog;
use figment::{Figment, providers::{Format, Toml, Env}};
use tui::{backend::CrosstermBackend, Terminal};

pub mod config;
pub mod discord;

use config::{Config, generate_config};

pub fn translate(cat: &Result<Catalog, gettext::Error>, msg: &str) -> String {
    if cat.is_ok() {
        let catalog = cat.as_ref().unwrap();
        return catalog.gettext(msg).to_string()
    }
    msg.to_string()
}

#[tokio::main]
async fn main() {
    let ru_cat_file = File::open("locales/ru.mo").expect("Cannot load `locales/ru.mo`");
    let catalog = Catalog::parse(ru_cat_file);

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    if !Path::new("config.toml").exists() {
        generate_config(&mut terminal, &catalog);
    }
    let config: Config = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("D2T_"))
        .extract().unwrap();

    let t1 = discord::start_discord_thread(config.discord_token).await;
    t1.join().unwrap().await;
}

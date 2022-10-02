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
use std::{io::{Stdout, Write}, time::Duration, fs::OpenOptions};
use crossterm::{event::{Event, KeyCode}, terminal};
use gettext::Catalog;
use serde::{Deserialize, Serialize};
use tui::{widgets::{Block, Borders, ListItem, List, ListState, Paragraph}, Terminal, backend::CrosstermBackend, layout::Rect};

use crate::translate;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub lang: String,
    pub discord_token: String,
    pub telegram_token: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: "en".to_string(),
            discord_token: String::new(),
            telegram_token: String::new()
        }
    }
}

#[derive(PartialEq)]
pub enum ConfigurationStage {
	Language,
	DiscordToken,
	TelegramToken
}

pub fn generate_config(term: &mut Terminal<CrosstermBackend<Stdout>>, catalog: &Result<Catalog, gettext::Error>) {
	// UI Variables
	let mut block = Block::default()
								.title("Select Display Language")
								.borders(Borders::ALL);
	let termsize = terminal::size().unwrap();
	let x = termsize.0 / 3;
	let y = termsize.1 / 3;
	let languages_rect = Rect::new(x, y, 30, 4);

	// Configuration Variables
	let mut config = Config::default();
	let mut config_stage = ConfigurationStage::Language;
	
	// Language Variables
	let mut languages_state = ListState::default();
	languages_state.select(Some(0));
	let languages = vec![ListItem::new("English"), ListItem::new("Русский")];
	let languages_size = languages.len();
	let languages_list = List::new(languages)
		.block(block)
		.highlight_symbol("> ");

	// Token variables
	let rect = Rect::new(x, y, 60, 4);

	let mut should_break = false;
	loop {
		term.draw(|frame| {
			// User Interface Handling
			match config_stage {
				ConfigurationStage::Language => frame.render_stateful_widget(languages_list.to_owned(), languages_rect, &mut languages_state),
				ConfigurationStage::DiscordToken => {
					let text = Paragraph::new(config.discord_token.as_ref())
													.block(Block::default()
																 .borders(Borders::ALL)
																 .title(translate(catalog, "Enter Discord Token:"))
														  );
					frame.render_widget(text, rect);
				},
				ConfigurationStage::TelegramToken => {
					let text = Paragraph::new(config.telegram_token.as_ref())
													.block(Block::default()
																 .borders(Borders::ALL)
																 .title(translate(catalog, "Enter Telegram Token:"))
														  );
					frame.render_widget(text, rect);
				}
			}

			// User Input Handling
			if crossterm::event::poll(Duration::from_millis(250)).unwrap() {
				if let Event::Key(key) = crossterm::event::read().unwrap() {
					match key.code {
						KeyCode::Esc => {
							std::process::exit(0);
						},
						KeyCode::Up => {
							if config_stage == ConfigurationStage::Language {
								let i = languages_state.selected().unwrap_or(0);
								let val = if i > 0 {
									i - 1
								} else {
									i
								};
								languages_state.select(Some(val));
							}
						},
						KeyCode::Down => {
							if config_stage == ConfigurationStage::Language {
								let i = languages_state.selected().unwrap_or(0);
								let val = if i < languages_size {
									i + 1
								} else {
									i
								};
								languages_state.select(Some(val));
							}
						},
						KeyCode::Char(c) => {
							if config_stage == ConfigurationStage::DiscordToken {
								config.discord_token.push(c);
							} else if config_stage == ConfigurationStage::TelegramToken {
								config.telegram_token.push(c);
							}
						},
						KeyCode::Enter => {
							match config_stage {
								ConfigurationStage::Language => {
									let i = languages_state.selected().unwrap_or(0);
									if i == 1 {
										config.lang = "ru".to_string();
									}

									config_stage = ConfigurationStage::DiscordToken;
								},
								ConfigurationStage::DiscordToken => {
									config_stage = ConfigurationStage::TelegramToken
								},
								ConfigurationStage::TelegramToken => {
									let toml = toml::to_string(&config).expect("Couldn't save configuration to disk");
									let mut file = OpenOptions::new()
															.create(true)
															.write(true)
															.open("config.toml").expect("Couldn't create config.toml");
									writeln!(&mut file, "{}", toml).expect("Couldn't write to config.toml");
									should_break = true;
								}
							}
						},
						_ => {}
					}
				}
			}
		}).unwrap();
		if should_break {
			break
		}
	}
}

/*pub fn generate_config(term: &mut Terminal<CrosstermBackend<Stdout>>) {
	let mut config_stage = 0;
	let mut list_state = ListState::default();
	list_state.select(Some(0));
	let languages = vec!["English", "Русский"];

	let mut config = Config::default();
	config.discord_token = "among".to_string();

	loop {
		term.draw(|f| {
			let mut target_string: &[u8] = config.discord_token.as_ref();
			let mut target_text = "Enter Discord Token:";
			if config_stage == 2 {
				target_string = config.telegram_token.as_ref();
				target_text = "Enter Telegram Token:";
			}
			match config_stage {
				0 => { 
					let items: Vec<ListItem> = languages.iter()
												.map(|i| ListItem::new(*i)).collect();
					let items = List::new(items)
											.block(Block::default().borders(Borders::ALL).title("Select Display Language"))
											.highlight_symbol(">> ");
					f.render_stateful_widget(items, f.size(), &mut list_state);
				},
				1 | 2 => {
					let target_string_len = target_string.len();
					let block = Block::default().title(translate(&config.lang, target_text)).borders(Borders::ALL);
					let input = Paragraph::new(String::from_utf8_lossy(target_string))
													.block(block);
					f.render_widget(input, f.size());

					f.set_cursor(
						1 + target_string_len as u16 + 1, 
						1
					);
				}
				_ => {}
			}

			if crossterm::event::poll(Duration::from_millis(250)).unwrap() {
				if let Event::Key(key) = crossterm::event::read().unwrap() {
					match key.code {
						KeyCode::Esc => std::process::exit(0),
						KeyCode::Up => {
							if config_stage == 0 {
								let i = match list_state.selected() {
									Some(i) => {
										if i >= languages.len() -1 {
											0
										} else {
											i + 1
										}
									},
									None => 0
								};
								list_state.select(Some(i));
							}
						},
						KeyCode::Down => {
							if config_stage == 0 {
								let i = match list_state.selected() {
									Some(i) => {
										if i == 0 {
											languages.len() - 1
										} else {
											i - 1
										}
									},
									None => 0
								};
								list_state.select(Some(i));
							}
						},
						KeyCode::Char(c) => {
							if c == 'q' && config_stage == 0 {
								std::process::exit(0);
							}

							
						},
						KeyCode::Backspace => {
							
						},
						KeyCode::Enter => {
							let i = match list_state.selected() {
								Some(i) => 
									i,
								None =>
									0
							};

							match config_stage {
								0 => {
									if i == 0 {
										config.lang = "en".to_string();
									} else {
										config.lang = "ru".to_string();
									}
								}
								_ => {}
							}
							if config_stage < 3 {
								config_stage += 1;
							}
						}
						_ => {}
					}
				}
			}
		}).unwrap();
	}
}*/
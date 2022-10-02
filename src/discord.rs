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
use std::thread;
use serenity::{prelude::{GatewayIntents, EventHandler}, Client};

pub struct Handler;
impl EventHandler for Handler {
	
}

pub async fn start_discord_thread(token: String) -> thread::JoinHandle<impl std::future::Future<Output = ()>> {
	thread::spawn(async || {
		let discord_intents = GatewayIntents::GUILD_MESSAGES |
                                         GatewayIntents::DIRECT_MESSAGES |
                                         GatewayIntents::MESSAGE_CONTENT;

		let mut discord_client = Client::builder(token, discord_intents).await.expect("Error while creating Discord Client. Is your token correct and is message content intent enabled?");

		if let Err(why) = discord_client.start().await {
			println!("Client error: {:?}", why);
		}
	})
}
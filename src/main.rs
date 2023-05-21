//! # Aegistrate - A moderation and general-purpose bot.
//!
//! I celebrated myself after coming up with that name. I'm that proud of
//! myself.
//!
//! Aegistrate is a (hopefully free) open-source project dedicated to being an
//! all-around, truly general-purposes bot, that users can configure and use
//! freely and directly without any strings attached.
//!
//! No in-app purchases (hopefully)!

#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(dead_code, clippy::module_name_repetitions)]

use std::env;

use aegis::{
	aegisize_unit,
	Aegis,
};
use handler::{
	spawn_timeout_checker,
	Handler,
};
use serenity::{
	prelude::GatewayIntents,
	Client,
};

pub mod aegis;
pub mod commands;
pub mod core;
pub mod handler;

/// Execution entry point of Aegistrate.
#[tokio::main]
async fn main() -> Aegis<()> {
	env::set_var("RUST_LOG", "serenity=debug,aegistrate=trace");
	dotenv::dotenv()?;
	env_logger::init();
	spawn_timeout_checker();

	let discord_bot_token = env::var("DISCORD_BOT_TOKEN")?;
	let mut discord_client = Client::builder(&discord_bot_token, GatewayIntents::all())
		.event_handler(Handler)
		.await?;
	aegisize_unit(discord_client.start().await)
}

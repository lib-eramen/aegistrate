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
use anyhow::bail;
use handler::{
	spawn_timeout_checker,
	Handler,
};
use log::error;
use serenity::{
	prelude::GatewayIntents,
	Client,
};

use crate::exec_config::ExecConfig;

pub mod aegis;
pub mod commands;
pub mod core;
pub mod data;
pub mod exec_config;
pub mod handler;

/// Execution entry point of Aegistrate.
#[tokio::main]
#[allow(clippy::cast_sign_loss)]
async fn main() -> Aegis<()> {
	env::set_var("RUST_LOG", "serenity=debug,aegistrate=trace");
	dotenv::dotenv()?;
	env_logger::init();

	let exec_config = ExecConfig::fetch();
	if exec_config.is_none() {
		error!(
			"No execution configuration files are found for Aegistrate! To get started, create a \
			 .env file in the repo directory, or create one in \
			 ~/.config/aegistrate/aegistrate.toml! See the documentation below for more info."
		);
		log!()
		bail!("No execution configuration files found.");
	}
	let exec_config = exec_config.unwrap();
	spawn_timeout_checker(exec_config.timeout_seconds as u64);

	let mut discord_client = Client::builder(&exec_config.discord_bot_token, GatewayIntents::all())
		.event_handler(Handler)
		.await?;
	aegisize_unit(discord_client.start().await)
}

#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(missing_docs, dead_code)]

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

mod aegis;
mod handler;

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

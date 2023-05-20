#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(missing_docs)]

use std::env;

use handler::Handler;
use serenity::{
	prelude::GatewayIntents,
	Client,
};

mod handler;

type Aegis<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Aegis<()> {
	env::set_var("RUST_LOG", "serenity=debug,sentinel=trace");
	dotenv::dotenv()?;
	env_logger::init();

	let discord_bot_token = env::var("DISCORD_BOT_TOKEN")?;
	let mut discord_client = Client::builder(&discord_bot_token, GatewayIntents::all())
		.event_handler(Handler)
		.await?;
	discord_client.start().await?;
	Ok(())
}

//! Outputs the "ping"/latency (in ms) that it takes to perform a request to
//! Discord's API. See also [Ping].

use std::time::Instant;

use async_trait::async_trait;
use serenity::{
	http::CacheHttp,
	model::prelude::interaction::application_command::ApplicationCommandInteraction,
	prelude::Context,
};

use crate::{
	aegis::{
		aegisize,
		Aegis,
	},
	bot::{
		commands::{
			components::embed::create_success_embed,
			util::message::{
				respond_with_embed,
				wait_a_moment,
				ResponseOptions,
			},
		},
		core::{
			command::{
				Command,
				Metadata,
			},
			plugin::Plugin,
		},
	},
};

/// The unit struct containing the implementation for the `/ping` command.
pub struct Ping;

#[async_trait]
impl Command for Ping {
	fn metadata(&self) -> Metadata<'_> {
		Metadata::builder()
			.name("ping")
			.description("Returns the ping of the bot. Pong!")
			.plugin(Plugin::Information)
			.cooldown_secs(0)
			.aliases(vec!["am-i-alive"])
			.build()
			.unwrap()
	}

	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()> {
		wait_a_moment(
			context.http(),
			interaction,
			ResponseOptions::CreateOrignial(false),
			Some("Pinging...".to_string()),
		)
		.await?;

		let ping_str = Self::get_ping_str().await;
		respond_with_embed(
			&context.http,
			interaction,
			ResponseOptions::EditOriginal,
			|embed| {
				create_success_embed(embed, "Pong!".to_string(), "I'm alive!".to_string()).field(
					"Latency",
					ping_str.clone(),
					true,
				)
			},
		)
		.await
		.map(|_| ())
	}
}
impl Ping {
	/// Retrieves the latency of the bot, by performing a GET request to the
	/// API. The currently used URL is [version 9's gateway](https://discordapp.com/api/v9/gateway).
	///
	/// # Errors
	///
	/// This function might return an [Error] if something happens while using
	/// [`reqwest::get`].
	pub async fn retrieve_latency() -> Aegis<u128> {
		let now = Instant::now();
		aegisize(
			reqwest::get("https://discordapp.com/api/v9/gateway").await,
			|_| now.elapsed().as_millis(),
		)
	}

	/// Returns a human-readable string of the latency retrieved, including an
	/// error message if something happens.
	pub async fn get_ping_str() -> String {
		Self::retrieve_latency().await.map_or_else(
			|_| "Unable to retrieve latency :(".to_string(),
			|latency| format!("{latency}ms"),
		)
	}
}

//! Provides convenience functions for registering commands to Discord's API.

use log::info;
use pluralizer::pluralize;
use serenity::{
	client::Cache,
	http::{
		CacheHttp,
		Http,
	},
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::core::{
		command::Command,
		plugin::get_guild_commands,
	},
	exec_config::get_working_guild_id,
};

/// Registers multiple commands to a guild.
#[allow(clippy::cast_possible_wrap)]
async fn register_commands(
	http: &Http,
	cache: &Cache,
	commands: Vec<Box<dyn Command>>,
) -> Aegis<()> {
	let guild = get_working_guild_id();
	let commands_count = commands.len();
	for command in commands {
		command.register_to_guild(http, cache).await?;
	}
	info!(
		"Guild \"{}\" ({}) registered {}",
		guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
		guild.0,
		pluralize("command", commands_count as isize, true),
	);
	Ok(())
}

/// Handles command registration for a guild, using the commands from the
/// guild's enabled plugins.
///
/// # Panics
///
/// This function will panic if the Discord context does not have a
/// functional cache.
///
/// # Errors
///
/// This function might fail if API calls to Discord fail as well.
pub async fn set_up_commands(context: &Context) -> Aegis<()> {
	let guild_id = get_working_guild_id();
	guild_id
		.set_application_commands(context.http(), |commands| {
			commands.set_application_commands(vec![])
		})
		.await?;

	let guild_commands = get_guild_commands().await?;
	register_commands(context.http(), context.cache().unwrap(), guild_commands).await
}

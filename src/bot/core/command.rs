//! Contains an abstraction for Discord's slash commands API, while alsoadding
//! semantic information into Aegistrate's commands to integrate with other
//! systems. To implement a command, get started with looking at the [Command]
//! trait.

use std::time::Duration;

use async_trait::async_trait;
use derive_builder::Builder;
use log::info;
use pluralizer::pluralize;
use serenity::{
	builder::CreateApplicationCommand,
	client::Cache,
	http::{
		CacheHttp,
		Http,
	},
	model::prelude::interaction::application_command::ApplicationCommandInteraction,
	prelude::Context,
};
use tokio::time::sleep;

use crate::{
	aegis::Aegis,
	bot::core::plugin::{
		get_guild_commands,
		Plugin,
	},
	exec_config::get_working_guild_id,
};

/// Alias for a [Vec] of [Box]ed [Command]s.
pub type Commands = Vec<Box<dyn Command>>;

/// Metadata associated with a command.
#[derive(Clone, Builder)]
pub struct Metadata<'a> {
	/// The name of the command.
	pub name: &'a str,

	/// The description of the command.
	pub description: &'a str,

	/// The plugin that the command belongs to.
	pub plugin: Plugin,

	/// The cooldown to use the command, in seconds.
	pub cooldown_secs: u64,

	#[builder(default)]
	/// The aliases for the command.
	pub aliases: Option<Vec<&'a str>>,

	#[builder(default)]
	/// The parameters that require non-Discord validation.
	pub validated_params: Option<ValidatedParameters<'a>>,
}

impl<'a> Metadata<'a> {
	/// Returns the [builder struct](MetadataBuilder) for this struct.
	#[must_use]
	pub fn builder<'b>() -> MetadataBuilder<'b> {
		MetadataBuilder::default()
	}

	/// Returns the list of all names and alises of this command.
	#[must_use]
	pub fn get_all_names(&self) -> Vec<&'a str> {
		if let Some(mut aliases) = self.aliases.clone() {
			aliases.push(self.name);
			aliases
		} else {
			vec![self.name]
		}
	}

	/// Returns the description of the command, noting the alias if the name
	/// happens to be one.
	#[must_use]
	pub fn get_description(&self, name: &str) -> String {
		format!(
			"{}{}",
			self.description,
			if self
				.aliases
				.as_ref()
				.is_some_and(|aliases| aliases.contains(&name))
			{
				format!(" Alias for /{}", self.name)
			} else {
				String::new()
			}
		)
	}
}

/// A metadata struct that specifies which parameter names requires validation
/// that Discord does not natively do.
#[derive(Clone, Default, Builder)]
pub struct ValidatedParameters<'a> {
	/// The parameters that are a date.
	pub dates: Option<Vec<&'a str>>,

	/// The parameters that are a time duration.
	pub durations: Option<Vec<&'a str>>,

	/// The parameters that are guild members.
	pub guild_members: Option<Vec<&'a str>>,
}

/// The command functionality to work with other systems in Aegistrate.
/// To make a command, implement this trait to your (hopefully unit) struct.
#[async_trait]
#[must_use]
pub trait Command: Send + Sync {
	/// Returns the metadata of the command.
	fn metadata(&self) -> Metadata<'_>;

	/// Registers the command's required metadata to the provided builder, in
	/// order to be sent to be registered via Discord's slash command API. Check
	/// out the [`CreateApplicationCommand`] struct for more information on how
	/// to implement this method.
	///
	/// Note to implementors: The command handler should have already registered
	/// the name, aliases and description for the command, so no worries doing
	/// that in this function.
	fn register<'a>(
		&self,
		command: &'a mut CreateApplicationCommand,
	) -> &'a mut CreateApplicationCommand {
		command
	}

	/// Executes this command.
	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()>;

	/// Registers all of this command's names and aliases.
	async fn register_to_guild<'a>(&self, http: &'a Http, cache: &'a Cache) -> Aegis<()> {
		let guild = get_working_guild_id();
		for name in self.metadata().get_all_names() {
			guild
				.create_application_command(http, |endpoint| {
					self.register(endpoint)
						.name(name)
						.description(self.metadata().get_description(name))
				})
				.await?;
		}
		info!(
			"Guild \"{}\" ({}) registered /{}",
			guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
			guild.0,
			self.metadata().name
		);
		Ok(())
	}
}

/// Returns all commands there are in Aegistrate.
/// Note that the commands are only retrieved via their [Plugin]s.
#[must_use]
pub fn all_commands() -> Commands {
	Plugin::commands_by_plugins(enum_iterator::all::<Plugin>().collect::<Vec<Plugin>>())
}

/// Returns a command with the name provided.
/// A [None] is returned if none is found.
#[must_use]
pub fn command_by_name(name: &str) -> Option<Box<dyn Command>> {
	all_commands()
		.into_iter()
		.find(|command| command.metadata().get_all_names().contains(&name))
}

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
		sleep(Duration::from_secs_f32(REGISTER_COMMAND_INTERVAL)).await;
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

/// The interval to sleep for between each command registration.
pub static REGISTER_COMMAND_INTERVAL: f32 = 1.0;

//! Contains an abstraction for Discord's slash commands API, while alsoadding
//! semantic information into Aegistrate's commands to integrate with other
//! systems. To implement a command, get started with looking at the [Command]
//! trait.

use async_trait::async_trait;
use derive_builder::Builder;
use log::info;
use serenity::{
	builder::CreateApplicationCommand,
	client::Cache,
	http::Http,
	model::prelude::{
		application_command::CommandDataOption,
		interaction::application_command::ApplicationCommandInteraction,
	},
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::core::{
		command::validate::{
			InvalidOptionError,
			ValidatedOptions,
		},
		plugin::Plugin,
	},
	exec_config::get_working_guild_id,
};

pub mod register;
pub mod validate;

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
	/// The parameters that require non-Discord validation.
	pub validated_options: ValidatedOptions<'a>,
}

impl<'a> Metadata<'a> {
	/// Returns the [builder struct](MetadataBuilder) for this struct.
	#[must_use]
	pub fn builder<'b>() -> MetadataBuilder<'b> {
		MetadataBuilder::default()
	}
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
		guild
			.create_application_command(http, |endpoint| {
				self.register(endpoint)
					.name(self.metadata().name)
					.description(self.metadata().description)
			})
			.await?;
		info!(
			"Guild \"{}\" ({}) registered /{}",
			guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
			guild.0,
			self.metadata().name
		);
		Ok(())
	}

	/// Validates this command's validatable options.
	fn validate(&self, options: &[CommandDataOption]) -> Result<(), InvalidOptionError> {
		self.metadata().validated_options.validate(options)
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
		.find(|command| command.metadata().name == name)
}

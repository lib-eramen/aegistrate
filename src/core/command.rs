//! Contains an abstraction for Discord's slash commands API, while alsoadding
//! semantic information into Aegistrate's commands to integrate with other
//! systems. To implement a command, get started with looking at the [Command]
//! trait.

use async_trait::async_trait;
use derive_builder::Builder;
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::interaction::application_command::ApplicationCommandInteraction,
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	core::plugin::Plugin,
};

/// Alias for a [Vec] of [Box]ed [Command]s.
pub type Commands = Vec<Box<dyn Command>>;

/// Metadata associated with a command.
#[derive(Clone, Builder)]
pub struct Metadata<'a> {
	/// The name of the command.
	pub name: &'a str,

	/// The plugin that the command belongs to.
	pub plugin: Plugin,

	/// The cooldown to use the command, in seconds.
	pub cooldown_secs: u64,

	/// The aliases for the command.
	pub aliases: Option<Vec<&'a str>>,
}

impl<'a> Metadata<'a> {
	/// Returns the [builder struct](MetadataBuilder) for this struct.
	#[must_use]
	pub fn builder<'b>() -> MetadataBuilder<'b> {
		MetadataBuilder::create_empty()
	}

	/// Returns the list of all names and alises of this command.
	#[must_use]
	pub fn all_names(&self) -> Vec<&'a str> {
		if let Some(mut aliases) = self.aliases.clone() {
			aliases.push(self.name);
			aliases
		} else {
			vec![self.name]
		}
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
	/// the name as well as the aliases for the command, so no worries doing
	/// that in this function.
	fn register<'a>(
		&self,
		command: &'a mut CreateApplicationCommand,
	) -> &'a mut CreateApplicationCommand;

	/// Executes this command.
	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()>;
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
		.find(|command| command.metadata().all_names().contains(&name))
}

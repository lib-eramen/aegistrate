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
#[derive(Copy, Clone, Builder)]
pub struct Metadata<'a> {
	/// The name of the command.
	pub name: &'a str,

	/// The plugin that the command belongs to.
	pub plugin: Plugin,

	/// The cooldown to use the command, in seconds.
	pub cooldown_secs: u64,
}

/// The command functionality to work with other systems in Aegistrate.
/// To make a command, implement this trait to your (hopefully unit) struct.
#[async_trait]
#[must_use]
pub trait Command {
	/// Returns the metadata of the command.
	fn metadata(&self) -> Metadata<'_>;

	/// Registers the command's required metadata to the provided builder, in
	/// order to be sent to be registered via Discord's slash command API. Check
	/// out the [`CreateApplicationCommand`] struct for more information on how
	/// to implement this method.
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

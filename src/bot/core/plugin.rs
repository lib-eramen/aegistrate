//! A system for organizing [Command](crate::core::command::Command)s in
//! Aegistrate, based on their semantic function. This system is mostly used to
//! prevent having too many commands at once per guild, as well as saves on
//! resources for Aegistrate.

#![allow(clippy::match_str_case_mismatch)]

use anyhow::bail;
use enum_iterator::{
	all,
	Sequence,
};
use serenity::{
	http::Http,
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::{
		commands::plugins::{
			information::information_commands,
			moderation::moderation_commands,
			plugins::plugin_commands,
		},
		core::command::{
			register::set_up_commands,
			Commands,
		},
		data::plugin::PluginData,
	},
	exec_config::get_working_guild_id,
};

/// A plugin that a command semantically belongs to.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence, Hash)]
pub enum Plugin {
	/// Performs moderation in the Discord guild.
	Moderation,

	/// Provides information about something.
	Information,

	/// Manipulates the plugin settings of a guild.
	Plugins,
}

impl Plugin {
	/// Converts an index (preferably obtained from [`enum_iterator::all`]) into
	/// this enum.
	#[must_use]
	pub fn from_index(index: usize) -> Option<Self> {
		all::<Self>().nth(index)
	}

	/// Converts this enum into an index, obtained from [`enum_iterator::all`].
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn to_index(self) -> usize {
		all::<Self>().position(|plugin| plugin == self).unwrap()
	}

	/// Converts this enum from a name.
	#[must_use]
	pub fn from_name(name: &str) -> Option<Self> {
		Some(match name {
			"Moderation" => Self::Moderation,
			"Information" => Self::Information,
			"Plugins" => Self::Plugins,
			_ => return None,
		})
	}

	/// Converts this enum to a name.
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn to_name(self) -> &'static str {
		match self {
			Self::Moderation => "Moderation",
			Self::Information => "Information",
			Self::Plugins => "Plugins",
		}
	}

	/// Gets a list of plugin names.
	#[must_use]
	pub fn get_plugin_names() -> Vec<&'static str> {
		all::<Self>().map(Self::to_name).collect()
	}

	/// Gets a list of commands that belongs to the current plugin.
	#[must_use]
	pub fn get_commands(self) -> Commands {
		match self {
			Self::Information => information_commands(),
			Self::Moderation => moderation_commands(),
			Self::Plugins => plugin_commands(),
		}
	}

	/// Gets a list of commands that belongs to one of the plugins provided.
	pub fn commands_by_plugins(plugins: Vec<Self>) -> Commands {
		plugins.into_iter().flat_map(Self::get_commands).collect()
	}

	/// Returns a list of default plugins. Default plugins cannot be disabled
	/// and is always on for all guilds when Aegistrate is added to the guild.
	#[must_use]
	pub fn default_plugins() -> Vec<Self> {
		vec![Self::Information, Self::Moderation, Self::Plugins]
	}

	/// Returns a list of non-default plugins.
	#[must_use]
	pub fn non_default_plugins() -> Vec<Self> {
		let default_plugins = Self::default_plugins();
		all::<Self>()
			.filter(|plugin| !default_plugins.contains(plugin))
			.collect()
	}

	/// Checks if the plugin is a default plugin.
	#[must_use]
	pub fn is_default(&self) -> bool {
		Self::default_plugins().contains(self)
	}

	/// Checks if the plugin requires setup before using its commands.
	#[must_use]
	pub fn requires_setup(&self) -> bool {
		false
	}

	/// Returns a list of default commands, by taking them from the [list of
	/// default plugins](Self::default_plugins).
	#[must_use]
	pub fn default_commands() -> Commands {
		Self::commands_by_plugins(Self::default_plugins())
	}
}

/// Gets all of the enabled commands for this guild.
///
/// # Errors
///
/// This function propagates errors from [`PluginData::get_enabled_commands`].
pub async fn get_guild_commands() -> Aegis<Commands> {
	PluginData::get_enabled_commands().await
}

/// Enables a plugin for a particular guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a plugin manager for
/// the provided guild ID, or if the provided plugin is already enabled for the
/// guild.
pub async fn enable_plugin(plugin: Plugin, http: &Http) -> Aegis<()> {
	if PluginData::get_enabled_plugins().await?.contains(&plugin) {
		bail!(
			"Plugin {} is already enabled for the current guild!",
			plugin.to_name()
		);
	}
	for command in plugin.get_commands() {
		get_working_guild_id()
			.create_application_command(http, |endpoint| command.register(endpoint))
			.await?;
	}
	PluginData::enable_plugin(plugin).await
}

/// Disables a plugin for a particular guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a plugin manager for
/// the provided guild ID, or if the provided plugin is not enabled for the
/// guild.
pub async fn disable_plugin(plugin: Plugin, context: &Context) -> Aegis<()> {
	if !PluginData::get_enabled_plugins().await?.contains(&plugin) {
		bail!(
			"Plugin {} is already disabled for the current guild!",
			plugin.to_name()
		);
	}
	PluginData::disable_plugin(plugin).await?;
	set_up_commands(context).await
}

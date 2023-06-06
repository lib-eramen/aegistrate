//! A system for organizing [Command](crate::core::command::Command)s in
//! Aegistrate, based on their semantic function. This system is mostly used to
//! prevent having too many commands at once per guild, as well as saves on
//! resources for Aegistrate.

use anyhow::bail;
use enum_iterator::{
	all,
	Sequence,
};

use crate::{
	aegis::Aegis,
	commands::plugins::information::information_commands,
	core::command::Commands,
	data::plugin::PluginManager,
};

/// A plugin that a command semantically belongs to.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence, Hash)]
pub enum Plugin {
	/// Indicates that the command performs moderation in the Discord guild.
	Moderation,

	/// Indicates that the command provides information about something.
	Information,
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
		Some(match name.to_lowercase().as_str() {
			"moderation" => Self::Moderation,
			"information" => Self::Information,
			_ => return None,
		})
	}

	/// Converts this enum to a name.
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn to_name(self) -> &'static str {
		match self {
			Self::Moderation => "moderation",
			Self::Information => "information",
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
			Self::Moderation => vec![],
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
		vec![Self::Information, Self::Moderation]
	}

	/// Checks if the plugin is a default plugin.
	#[must_use]
	pub fn is_default(&self) -> bool {
		Self::default_plugins().contains(self)
	}

	/// Returns a list of default commands, by taking them from the [list of
	/// default plugins](Self::default_plugins).
	#[must_use]
	pub fn default_commands() -> Commands {
		Self::commands_by_plugins(Self::default_plugins())
	}
}

/// Gets the plugin manager for a guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a plugin manager for
/// the provided guild ID.
pub async fn get_plugin_manager(guild_id: u64) -> Aegis<PluginManager> {
	PluginManager::find_one(guild_id).await
}

/// Gets all of the enabled commands for a particular guild.
///
/// # Panics
///
/// This function will panic if unable to find a plugin manager for the provided
/// guild ID.
pub async fn get_guild_commands(guild_id: u64) -> Commands {
	get_plugin_manager(guild_id)
		.await
		.unwrap()
		.get_enabled_commands()
}

/// Enables a plugin for a particular guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a plugin manager for
/// the provided guild ID, or if the provided plugin is already enabled for the
/// guild.
pub async fn enable_plugin(guild_id: u64, plugin: Plugin) -> Aegis<()> {
	let mut plugin_manager = get_plugin_manager(guild_id).await?;
	if plugin_manager.get_enabled_plugins().contains(&plugin) {
		bail!(
			"Plugin {} is already enabled for guild {guild_id}!",
			plugin.to_name()
		);
	}
	plugin_manager.enable_plugin(plugin).await
}

/// Disables a plugin for a particular guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a plugin manager for
/// the provided guild ID, or if the provided plugin is not enabled for the
/// guild.
pub async fn disable_plugin(guild_id: u64, plugin: Plugin) -> Aegis<()> {
	let mut plugin_manager = get_plugin_manager(guild_id).await?;
	if !plugin_manager.get_enabled_plugins().contains(&plugin) {
		bail!(
			"Plugin {} is already disabled for guild {guild_id}!",
			plugin.to_name()
		);
	}
	plugin_manager.disable_plugin(plugin).await
}

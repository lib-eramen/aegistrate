//! A system for organizing [Command](crate::core::command::Command)s in
//! Aegistrate, based on their semantic function. This system is mostly used to
//! prevent having too many commands at once per guild, as well as saves on
//! resources for Aegistrate.

use enum_iterator::{
	all,
	Sequence,
};

use crate::{
	commands::plugins::information::information_commands,
	core::command::Commands,
};

/// A plugin that a command semantically belongs to.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence)]
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
		vec![Self::Moderation]
	}

	/// Returns a list of default commands, by taking them from the [list of
	/// default plugins](Self::default_plugins).
	#[must_use]
	pub fn default_commands() -> Commands {
		Self::commands_by_plugins(Self::default_plugins())
	}
}

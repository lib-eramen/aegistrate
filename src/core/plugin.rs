//! A system for organizing [Command](crate::core::command::Command)s in
//! Aegistrate, based on their semantic function. This system is mostly used to
//! prevent having too many commands at once per guild, as well as saves on
//! resources for Aegistrate.

use enum_iterator::{
	all,
	Sequence,
};

use crate::core::command::Commands;

// TODO: Command creation
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

	// TODO
	/// Returns whether this plugin that the enum represents can be disabled.
	/// Note that this is a manually maintained and updated check.
	#[must_use]
	pub fn can_be_disabled(self) -> bool {
		false
	}

	/// Gets a list of plugin names.
	#[must_use]
	pub fn get_plugin_names() -> Vec<&'static str> {
		all::<Self>().map(Self::to_name).collect()
	}

	/// Gets a list of 
	#[must_use]
	pub fn get_commands(self) -> Commands {
		match self {
			Self::Information | Self::Moderation => vec![],
		}
	}

	pub fn commands_by_plugins(plugins: Vec<Self>) -> Commands {
		plugins.into_iter().flat_map(Self::get_commands).collect()
	}

	#[must_use]
	pub fn default_plugins() -> Vec<Self> {
		vec![Self::Moderation]
	}

	#[must_use]
	pub fn default_commands() -> Commands {
		Self::commands_by_plugins(Self::default_plugins())
	}
}

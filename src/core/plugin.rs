use enum_iterator::{
	all,
	Sequence,
};

use crate::core::command::Commands;

/// can boost the functionality of another.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Sequence)]
pub enum Plugin {
	Moderation,
}

impl Plugin {
	#[must_use]
	pub fn from_index(index: usize) -> Option<Self> {
		all::<Self>().nth(index)
	}

	#[must_use]
	pub fn to_index(self) -> usize {
		all::<Self>().position(|plugin| plugin == self).unwrap()
	}

	#[must_use]
	pub fn from_name(name: &str) -> Option<Self> {
		Some(match name.to_lowercase().as_str() {
			"moderation" => Self::Moderation,
			_ => return None,
		})
	}

	#[must_use]
	pub fn to_name(self) -> &'static str {
		match self {
			Self::Moderation => "moderation",
		}
	}

	#[must_use]
	pub fn can_be_disabled(self) -> bool {
		todo!()
	}

	#[must_use]
	pub fn get_plugin_names() -> Vec<&'static str> {
		all::<Self>().map(Self::to_name).collect()
	}

	#[must_use]
	pub fn get_commands(self) -> Commands {
		match self {
			Self::Moderation => todo!(),
		}
	}

	pub fn commands_by_plugins(plugins: Vec<Self>) -> Commands {
		plugins.into_iter().flat_map(Self::get_commands).collect()
	}

	#[must_use]
	pub fn default_plugins() -> Vec<Self> {
		vec![Self::Moderation]
	}

	/// Gets a [`Vec`] of commands that belong to the [default
	/// plugins](Self::default_plugins).
	#[must_use]
	pub fn default_commands() -> Commands {
		Self::commands_by_plugins(Self::default_plugins())
	}
}

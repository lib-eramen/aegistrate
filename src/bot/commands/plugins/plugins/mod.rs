//! Contains commands that manipulates plugin settings of a guild.

use crate::bot::{
	commands::plugins::plugins::{
		disable::Disable,
		enable::Enable,
	},
	core::command::Commands,
};

/// Returns all commads belonging to the [plugin](self) plugin.
#[must_use]
pub fn plugin_commands() -> Commands {
	vec![Box::new(Enable), Box::new(Disable)]
}

pub mod disable;
pub mod enable;
// pub mod plugins;

//! Contains commands that manipulates plugin settings of a guild.

use crate::{
	commands::plugins::plugins::enable_plugin::EnablePlugin,
	core::command::Commands,
};

/// Returns all commads belonging to the [plugin](self) plugin.
#[must_use]
pub fn plugin_commands() -> Commands {
	vec![Box::new(EnablePlugin)]
}

// pub mod disable_plugin;
pub mod enable_plugin;
// pub mod enabled_plugins;

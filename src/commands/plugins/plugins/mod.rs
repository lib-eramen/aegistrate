//! Contains commands that manipulates plugin settings of a guild.

use crate::{
	commands::plugins::plugins::enable::Enable,
	core::command::Commands,
};

/// Returns all commads belonging to the [plugin](self) plugin.
#[must_use]
pub fn plugin_commands() -> Commands {
	vec![Box::new(Enable)]
}

// pub mod disable_plugin;
pub mod enable;
// pub mod enabled_plugins;

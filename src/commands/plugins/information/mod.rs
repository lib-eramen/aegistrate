//! Contains commands that outputs information that users can't trivially get.
//! This can be anything - the Aegistrate bot's data/statistics, member data,
//! etc.

use crate::{
	commands::plugins::information::ping::Ping,
	core::command::Commands,
};

pub mod ping;

/// Returns all commads belonging to the [information](self) plugin.
#[must_use]
pub fn information_commands() -> Commands {
	vec![Box::new(Ping)]
}

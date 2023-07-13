//! Contains commands that execute moderation actions in Discord.
//! Can be a wrapper for things like `/ban`, `/kick`, `/timeout`,
//! but also more advanced ones like `/soft-ban`, etc.

use crate::bot::{
	commands::plugins::moderation::ban::Ban,
	core::command::Commands,
};

pub mod ban;

/// Returns all commads belonging to the [moderation](self) plugin.
#[must_use]
pub fn moderation_commands() -> Commands {
	vec![Box::new(Ban)]
}

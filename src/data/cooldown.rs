//! Handles data for the [cooldown](crate::core::cooldown) feature.
//! This is not the place to look for the API of said feature - the linked
//! module is a good place for that.

use std::collections::HashMap;

use mongod::{
	AsUpdate,
	Bson,
	Mongo,
};

use crate::{common_db_impl, aegis::Aegis};

/// A struct that manages cooldowns for guilds.
/// For how executions are stored, the string is stored in the form that looks
/// like: `"{user-id}:{command-name}"`. This is used to deal with [`mongod`]
/// limitations.
#[derive(Bson, Mongo, Clone, Default)]
#[mongo(collection = "cooldown", field, filter, update)]
#[must_use]
#[rustfmt::skip]
pub struct CooldownManager {
	/// The guild ID that this struct manages.
	pub guild_id: u64,

	/// Contains all of the guild's last command executions, the key being the command name & the user ID (see [this](self) on the schema for the name!) and the value being the UNIX timestamp of the last command execution.
	pub executions: HashMap<String, u64>,
}

common_db_impl!(CooldownManager, self, {
	let mut update = Self::update();
	update.executions = Some(self.executions.clone());
	update
});

/// Formats a user and a command name into an entry key for [`CooldownManager`].
#[must_use]
pub fn user_cmd_key_str(user_id: u64, cmd_name: &str) -> String {
	format!("{user_id}:{cmd_name}")
}

impl CooldownManager {
	/// Returns an [Option]al last use of a command.
	#[must_use]
	pub fn get_last_use(&self, user_id: u64, cmd_name: &str) -> Option<u64> {
		self.executions.get(&user_cmd_key_str(user_id, cmd_name)).copied()
	}

	/// Creates an entry for the last use of a command, while also updating
	/// it to the database. 
	/// TODO: API wrapper at the [`crate::core::cooldown`] module
	pub(crate) async fn create_last_use(&mut self, user_id: u64, cmd_name: &str, timestamp: u64) -> Aegis<()> {
		self.executions.insert(user_cmd_key_str(user_id, cmd_name), timestamp);
		self.update_entry().await
	}

}

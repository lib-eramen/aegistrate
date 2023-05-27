//! Handles data for the [cooldown](crate::core::cooldown) feature.
//! This is not the place to look for the API of said feature - the linked
//! module is a good place for that.

use std::collections::HashMap;

use mongod::{
	AsUpdate,
	Bson,
	Mongo,
};

use crate::common_db_impl;

/// A struct that manages cooldowns for guilds.
/// For how executions are stored, the string is stored in the form that looks
/// like: `"{user-id}/{command-name}"`. This is used to deal with [`mongod`]
/// limitations.
#[derive(Bson, Mongo, Clone, Default)]
#[mongo(collection = "cooldown", field, filter, update)]
#[rustfmt::skip]
pub struct CooldownManager {
	/// The guild ID that this struct manages.
	pub guild_id: u64,

	/// Contains all of the guild's last command executions, the key being the command name & the user ID (see [this](self) on the schema for the name!) and the value being the UNIX  of the last command execution.
	pub executions: HashMap<String, u64>,
}

common_db_impl!(CooldownManager, self, {
	let mut update = Self::update();
	update.executions = Some(self.executions.clone());
	update
});

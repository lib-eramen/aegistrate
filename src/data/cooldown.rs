//! Handles data for the [cooldown](crate::core::cooldown) feature.
//! This is not the place to look for the API of said feature - the linked
//! module is a good place for that.

use derive_new::new;
use mongod::{
	AsFilter,
	AsUpdate,
	Bson,
	Comparator,
	Mongo,
};

use crate::{
	aegis::{
		aegisize,
		Aegis,
	},
	common_db_impl,
	handler::get_mongodb_client,
};

/// A struct that contains cooldown data for guilds.
/// For how executions are stored, the string is stored in the form that looks
/// like: `"{user-id}:{command-name}"`. This is used to deal with [`mongod`]
/// limitations.
#[derive(Bson, Mongo, Clone, Default, new)]
#[mongo(collection = "command-executions", field, filter, update)]
#[must_use]
#[rustfmt::skip]
pub struct CooldownData {
	/// The command name & user ID. See [this](self) on how this is stored.
	pub name_and_user: String,

	/// The UNIX timestamp of the last command execution corresponding to the name and user.
	pub timestamp: u64,
}

common_db_impl!(
	CooldownData,
	self,
	{
		let mut update = Self::update();
		update.name_and_user = Some(self.name_and_user.clone());
		update.timestamp = Some(self.timestamp);
		update
	},
	{
		let mut filter = Self::filter();
		filter.name_and_user = Some(Comparator::Eq(self.name_and_user.clone()));
		filter
	}
);

/// Formats a user and a command name into an entry key for [`CooldownManager`].
#[must_use]
pub fn user_cmd_key_str(user_id: u64, cmd_name: &str) -> String {
	format!("{user_id}:{cmd_name}")
}

impl CooldownData {
	/// Returns an [Option]al last use of a command.
	///
	/// # Errors
	///
	/// This function will fail if I/O returns an error.
	pub async fn get_last_use(user_id: u64, cmd_name: &str) -> Aegis<Option<u64>> {
		let mut filter = Self::filter();
		filter.name_and_user = Some(Comparator::Eq(user_cmd_key_str(user_id, cmd_name)));
		aegisize(
			get_mongodb_client().find_one::<Self, _>(filter).await,
			|result| result.map(|data| data.1.timestamp),
		)
	}

	/// Creates an entry for the last use of a command, while also updating
	/// it to the database.
	pub(crate) async fn create_last_use(user_id: u64, cmd_name: &str, timestamp: u64) -> Aegis<()> {
		let new_cooldown = Self::new(user_cmd_key_str(user_id, cmd_name), timestamp);
		if new_cooldown.filter_one_by_self().await?.is_none() {
			get_mongodb_client()
				.insert_one::<Self>(Self::new(user_cmd_key_str(user_id, cmd_name), timestamp))
				.await?;
		} else {
			new_cooldown.update_entry().await?;
		}
		Ok(())
	}
}

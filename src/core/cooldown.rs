//! The cooldown feature, used to control the frequency of commands being used.
//! It is mostly used to prevent API abuse.
//!
//! This module wraps [`CooldownManager`]. If one wants to work with the
//! underlying layer of this API, see [`crate::data::cooldown`] and the
//! aforementioned struct.

use std::time::{
	SystemTime,
	UNIX_EPOCH,
};

use crate::{
	aegis::Aegis,
	core::command::Command,
	data::cooldown::CooldownManager,
};

/// Gets the cooldown manager for a guild.
///
/// # Errors
///
/// This function will return an [Err] if unable to find a cooldown manager for
/// the provided guild ID.
pub async fn get_cooldown_manager() -> Aegis<CooldownManager> {
	CooldownManager::find_one().await
}

/// Gets the remaining seconds to wait for a cooldown to finish.
///
/// # Errors
///
/// This function propagates errors from [`get_cooldown_manager`].
///
/// # Panics
///
/// This function panics if [`SystemTime::now`] happens to be earlier than
/// [`UNIX_EPOCH`], which, why, you sneaky bastard?
pub async fn get_remaining_cooldown(user_id: u64, command: &dyn Command) -> Aegis<u64> {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();
	let manager = get_cooldown_manager().await?;
	let last_use = manager.get_last_use(user_id, command.metadata().name);
	let cooldown = command.metadata().cooldown_secs;

	Ok(if let Some(last_use) = last_use {
		let between_period = now - last_use;
		if between_period >= cooldown {
			0
		} else {
			cooldown - between_period
		}
	} else {
		0
	})
}

/// Checks if a [command](Command) has completely cooled down and ready for
/// use again. Note that this function assumes that a cooldown manager exists,
/// and if one does not exist, the function will return `false`.
///
/// # Errors
///
/// This function will propagate errors from [`get_remaining_cooldown`].
pub async fn cooled_down(user_id: u64, command: &dyn Command) -> Aegis<bool> {
	get_remaining_cooldown(user_id, command)
		.await
		.map(|cooldown| cooldown == 0)
}

/// Set a last use for a command.
///
/// # Errors
///
/// This function propagates errors from [`get_cooldown_manager`].
///
/// # Panics
///
/// This function panics if [`SystemTime::now`] happens to be earlier than
/// [`UNIX_EPOCH`], which, I don't know how that is possible.
pub async fn use_last(user_id: u64, command: &dyn Command) -> Aegis<()> {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();
	get_cooldown_manager()
		.await?
		.create_last_use(user_id, command.metadata().name, now)
		.await
}

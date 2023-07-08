//! Module containing everything to do with data processing and interacting with
//! the database for Aegistrate.
//!
//! If you are looking for a particular mechanism/feature, you are probably
//! looking at the wrong place. For those APIs (that wrap this module) that
//! Aegistrate would use over ones provided here, it is at [`crate::core`].

use crate::{
	aegis::Aegis,
	data::{
		cooldown::CooldownManager,
		plugin::PluginManager,
	},
};

pub mod cooldown;
pub mod plugin;

// TODO: Plugin system integration
/// Initializes the various data structs for a guild ID.
///
/// # Errors
///
/// This function inherits errors from the various functions that interact via
/// I/O with MongoDB.
pub async fn init_all_data() -> Aegis<()> {
	CooldownManager::create_default_if_not_found().await?;
	PluginManager::create_default_if_not_found().await
}

/// Implements several functions for a given struct that derives
/// [`mongod::Mongo`] and [Default].
///
/// Note that this macro also adds two function-local `use`s,
/// [`mongod::AsFilter`] and [`mongod::AsUpdate`]. It also requires the user to
/// provide an expression that will create a [`mongod::Update`] object from the
/// struct.
///
/// Clippy might also complain about `[must_use]` if the provided database
/// struct is not already `[must_use]`.
///
/// Highly dirty and janky. Never use outside of Aegistrate.
#[macro_export]
macro_rules! common_db_impl {
	($name: ident, $self: ident, $update_code: expr) => {
		impl $name {
			/// Creates a default instance of this struct and writes it to the database.
			///
			/// # Errors
			///
			/// This function will return with an error if the guild ID provided matches
			/// another instance of this struct already in the database.
			pub async fn create_default() -> $crate::Aegis<Self> {
				if Self::search().await.unwrap().is_some() {
					anyhow::bail!(
						"An instance of this struct for this guild already exists!"
					);
				}
				$crate::handler::get_mongodb_client()
					.insert_one::<Self>(Self::default())
					.await?;
				Ok(Self::default())
			}

			/// Gets an entry from the database that matches the given guild ID,
			/// or return [None] if none exists.
			///
			/// # Errors
			///
			/// This function will propagate I/O errors from querying the database.
			pub async fn search() -> $crate::Aegis<Option<Self>> {
				use mongod::AsFilter;
				Ok($crate::handler::get_mongodb_client()
					.find_one::<Self, _>(Self::filter())
					.await?
					.map(|result| result.1))
			}

			/// Creates a default entry for the guild ID if not already found.
			///
			/// # Errors
			///
			/// This function will propagate errors from [`Self::create_default`].
			pub async fn create_default_if_not_found() -> $crate::Aegis<()> {
				// Self::create_default will have bail!ed if an instance was found.
				let _ = Self::create_default().await;
				Ok(())
			}

			/// Gets an entry from the database that matches the given guild ID.
			/// Note that this function assumes that there is already an existing entry
			/// with the specified guild ID, and uses [`Option::unwrap`].
			///
			/// # Errors
			///
			/// This function will propagate errors from [`Self::search`].
			pub async fn find_one() -> $crate::Aegis<Self> {
				Self::search().await.map(Option::unwrap)
			}

			/// Gets an existing entry, or creates a default one if an entry was not found.
			///
			/// # Errors
			///
			/// This function will propagate errors from [`Self::search`] and [`Self::create_default`].
			pub async fn find_or_create_default() -> $crate::Aegis<Self> {
				let result = Self::search().await?;
				Ok(if result.is_none() {
					Self::create_default().await?
				} else {
					result.unwrap()
				})
			}

			/// Updates an entry on the database, based on the instance calling this
			/// method.
			///
			/// # Errors
			///
			/// This function will propagate I/O errors from the database client.
			pub async fn update_entry(&$self) -> $crate::Aegis<()> {
				use mongod::AsFilter;
				$crate::aegis::aegisize_unit(
					$crate::handler::get_mongodb_client()
						.update::<Self, _, _>(Self::filter(), mongod::Updates {
							set: Some({ $update_code }),
							unset: None,
						})
						.await,
				)
			}
		}
	};
}

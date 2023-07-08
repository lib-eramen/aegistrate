//! Module containing everything to do with data processing and interacting with
//! the database for Aegistrate.
//!
//! If you are looking for a particular mechanism/feature, you are probably
//! looking at the wrong place. For those APIs (that wrap this module) that
//! Aegistrate would use over ones provided here, it is at [`crate::core`].

use crate::{
	aegis::Aegis,
	data::plugin::PluginData,
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
	PluginData::prepare_data().await
}

/// Implements several functions for a given struct that derives
/// [`mongod::Mongo`] and [Default].
///
/// Note that this macro also adds two function-local `use`s,
/// [`mongod::AsFilter`] and [`mongod::AsUpdate`]. It also requires the user to
/// provide an expression that will create a [`mongod::Update`] object from the
/// struct. In addition, the user also has to provide an expression that will
/// create a [`mongod::Filter`] object from the struct.
///
/// Clippy might also complain about `[must_use]` if the provided database
/// struct is not already `[must_use]`.
///
/// Highly dirty and janky. Never use outside of Aegistrate.
#[macro_export]
macro_rules! common_db_impl {
	($name: ident, $self: ident, $update_code: expr, $filter_code: expr) => {
		impl $name {
			/// Returns if the specified collection associated with the current struct
			/// has entries.
			///
			/// # Errors
			///
			/// This function will fail if I/O goes wrong.
			pub async fn contains_entries() -> $crate::Aegis<bool> {
				Self::search_one().await.map(|result| result.is_some())
			}

			/// Gets an entry from the database, or return [None] if none exists.
			///
			/// # Errors
			///
			/// This function will propagate I/O errors from querying the database.
			pub async fn search_one() -> $crate::Aegis<Option<Self>> {
				use mongod::AsFilter;
				Ok($crate::handler::get_mongodb_client()
					.find_one::<Self, _>(Self::filter())
					.await?
					.map(|result| result.1))
			}

			/// Gets an entry from the database that matches the filter given, based on `self`'s own data.
			///
			/// # Errors
			///
			/// This function will propagate I/O errors from querying the database.
			pub async fn filter_one_by_self(&$self) -> $crate::Aegis<Option<Self>> {
				use mongod::AsFilter;
				Ok($crate::handler::get_mongodb_client()
					.find_one::<Self, _>({ $filter_code })
					.await?
					.map(|data| data.1))
			}

			/// Gets all entries from the collection.
			///
			/// # Errors
			///
			/// This function will return errors from database I/O.
			pub async fn find_all() -> $crate::Aegis<Vec<Self>> {
				use futures::StreamExt;
				let mut query_stream = $crate::handler::get_mongodb_client()
					.find::<Self, _>(None).await?
					.map(|item| item.unwrap().1);
				let mut results = vec![];
				while let Some(item) = query_stream.next().await {
					results.push(item);
				}
				Ok(results)
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
						.update_one::<Self, _, _>(Self::filter(), mongod::Updates {
							set: Some({ $update_code }),
							unset: None,
						})
						.await,
				)
			}
		}
	};
}

//! Handles data for the [plugin](crate::core::plugin) feature.
//! This is not the place to look for the API of said feature - the linked
//! module is a good place for that.

use std::collections::HashSet;

use derive_new::new;
use mongod::{
	AsFilter,
	AsUpdate,
	Bson,
	Comparator,
	Mongo,
};

use crate::{
	aegis::Aegis,
	bot::{
		core::{
			command::Commands,
			plugin::Plugin,
		},
		handler::get_mongodb_client,
	},
	common_db_impl,
};

/// An entry for an enabled plugin, stored by their name.
#[derive(Bson, Mongo, Clone, Default, new)]
#[mongo(collection = "enabled-plugins", field, filter, update)]
#[must_use]
#[rustfmt::skip]
pub struct PluginData {
    /// The guild's enabled plugin name.
    pub name: String,
}

common_db_impl!(
	PluginData,
	self,
	{
		let mut update = Self::update();
		update.name = Some(self.name.clone());
		update
	},
	{
		let mut filter = Self::filter();
		filter.name = Some(Comparator::Eq(self.name.clone()));
		filter
	}
);

impl PluginData {
	/// Creates the default data associated with the plugin system.
	/// This function is intended to be called when Aegistrate starts up.
	///
	/// # Errors
	///
	/// This function may fail if I/O errors happen while quering the database.
	pub async fn prepare_data() -> Aegis<()> {
		if Self::contains_entries().await? {
			return Ok(());
		}
		for plugin in Plugin::default_plugins() {
			Self::enable_plugin(plugin).await?;
		}
		Ok(())
	}

	/// Returns the list of plugins that this guild has enabled.
	///
	/// # Errors
	///
	/// This function will fail if an I/O error happens.
	///
	/// # Panics
	///
	/// This function will panic if it discovers an entry with an invalid plugin
	/// name.
	pub async fn get_enabled_plugins() -> Aegis<HashSet<Plugin>> {
		Ok(PluginData::find_all()
			.await?
			.into_iter()
			.map(|name| Plugin::from_name(&name.name).unwrap())
			.chain(Plugin::default_plugins())
			.collect())
	}

	/// Returns the list of commands that are enabled for this guild.
	///
	/// # Errors
	///
	/// This function propagates errors from [`Self::get_enabled_plugins`].
	pub async fn get_enabled_commands() -> Aegis<Commands> {
		Ok(Self::get_enabled_plugins()
			.await?
			.into_iter()
			.flat_map(Plugin::get_commands)
			.collect())
	}

	/// Inserts a plugin into the list of enabled plugins.
	///
	/// # Errors
	///
	/// This function might return an [Err] if something happens during I/O to
	/// the database.
	pub async fn enable_plugin(plugin: Plugin) -> Aegis<()> {
		get_mongodb_client()
			.insert_one::<Self>(Self::new(plugin.to_name().to_string()))
			.await?;
		Ok(())
	}

	/// Removes a plugin from the list of enabled plugins.
	///
	/// # Errors
	///
	/// This function might return an [Err] if something happens during I/O to
	/// the database.
	pub async fn disable_plugin(plugin: Plugin) -> Aegis<()> {
		let mut filter = Self::filter();
		filter.name = Some(Comparator::Eq(plugin.to_name().to_string()));
		get_mongodb_client().delete_one::<Self, _>(filter).await?;
		Ok(())
	}
}

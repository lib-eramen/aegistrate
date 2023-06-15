//! Handles data for the [plugin](crate::core::plugin) feature.
//! This is not the place to look for the API of said feature - the linked
//! module is a good place for that.

use std::collections::HashSet;

use mongod::{
	AsUpdate,
	Bson,
	Mongo,
};

use crate::{
	aegis::Aegis,
	common_db_impl,
	core::{
		command::Command,
		plugin::Plugin,
	},
};

/// A struct that manages plugins for guilds.
/// Plugins are stored by their names.
#[derive(Bson, Mongo, Clone, Default)]
#[mongo(collection = "plugin", field, filter, update)]
#[must_use]
#[rustfmt::skip]
pub struct PluginManager {
    /// The guild ID that this struct manages.
    pub guild_id: u64,

    /// The guild's enaled plugin names.
    pub enabled_plugin_names: HashSet<String>,
}

common_db_impl!(PluginManager, self, {
	let mut update = Self::update();
	update.enabled_plugin_names = Some(self.enabled_plugin_names.clone());
	update
});

impl PluginManager {
	/// Returns the list of plugins that this guild has enabled.
	///
	/// # Panics
	///
	/// If the names contained in this struct is not a valid plugin name, the
	/// function will panic.
	#[must_use]
	pub fn get_enabled_plugins(&self) -> HashSet<Plugin> {
		self.enabled_plugin_names
			.iter()
			.map(|name| Plugin::from_name(name).unwrap())
			.chain(Plugin::default_plugins())
			.collect()
	}

	/// Returns the list of commands that are enabled for this guild.
	#[must_use]
	pub fn get_enabled_commands(&self) -> Vec<Box<dyn Command>> {
		self.get_enabled_plugins()
			.into_iter()
			.flat_map(Plugin::get_commands)
			.collect()
	}

	/// Inserts a plugin into the list of enabled plugins.
	///
	/// # Errors
	///
	/// This function might return an [Err] if something happens during I/O to
	/// the database.
	pub async fn enable_plugin(&mut self, plugin: Plugin) -> Aegis<()> {
		self.enabled_plugin_names
			.insert(plugin.to_name().to_string());
		self.update_entry().await
	}

	/// Removes a plugin from the list of enabled plugins.
	///
	/// # Errors
	///
	/// This function might return an [Err] if something happens during I/O to
	/// the database.
	pub async fn disable_plugin(&mut self, plugin: Plugin) -> Aegis<()> {
		self.enabled_plugin_names
			.remove(&plugin.to_name().to_string());
		self.update_entry().await
	}
}

//! Controls the configuration for the execution of Aegistrate, **NOT** the
//! application's preferences.

use std::{
	env::var,
	fs::{
		create_dir_all,
		read_to_string,
		OpenOptions,
	},
	io::Write,
	path::PathBuf,
};

use anyhow::bail;
use log::{
	error,
	info,
};
use serde::{
	Deserialize,
	Serialize,
};
use serenity::model::prelude::GuildId;
use tokio::sync::OnceCell;

use crate::aegis::{
	aegisize_unit,
	Aegis,
};

/// Aegistrate's global execution configuration.
pub static GLOBAL_EXEC_CONFIG: OnceCell<ExecConfig> = OnceCell::const_new();

/// Aegistrate's preferred path to its execution configuration.
///
/// Note that this path is **NOT COMPLETE** - one has to join the home directory
/// with this path to work. Use the path returned by
/// [`get_preferred_config_path`] for convenience.
pub static PREFERRED_CONFIG_PATH: &str = ".config/aegistrate/aegistrate.toml";

/// Gets the complete preferred config path for Aegistrate.
///
/// # Panics
///
/// This function will panic if [`home::home_dir`] returns a [None] (which is
/// unwrapped)
#[must_use]
pub fn get_preferred_config_path() -> PathBuf {
	home::home_dir().unwrap().join(PREFERRED_CONFIG_PATH)
}

/// Initializes the [`EXEC_CONFIG`] static variable.
///
/// # Errors
///
/// This function will return an error when no execution configuration files
/// were found.
#[allow(clippy::missing_panics_doc)]
pub fn initialize_exec_config() -> Aegis<()> {
	let exec_config = ExecConfig::fetch();
	if exec_config.is_none() {
		let preferred_config_path = get_preferred_config_path();
		let display_path = preferred_config_path.display();
		error!("No valid execution configuration files are found for Aegistrate!");
		error!(
			"If there exists an execution configuration file for Aegistrate, check to see if it \
			 has all the keys required for Aegistrate to run. Refer to docs/EXEC-CONFIG.md for \
			 more info. More information can also be found at the GitHub repo: https://github.com/developer-ramen/aegistrate"
		);
		error!(
			"Otherwise, create a .env file in the repo directory, or create one in \
			 {display_path}! See the documentation in docs/EXEC-CONFIG.md for more info."
		);
		error!(
			"Aegistrate should have created a config file at {display_path} when this error was \
			 generated. If you see an error similar to \"Error 21: Is a directory\", check if the \
			 file at the end of this path: {display_path} is actually a file."
		);
		create_default_exec_config().map(|_| {
			info!(
				"Created an exec-config file at {display_path}. This file should contain some \
				 basic starter options so that you can fill it in and get started with running \
				 Aegistrate."
			);
		})?;
		bail!("No execution configuration files found.");
	}
	aegisize_unit(GLOBAL_EXEC_CONFIG.set(exec_config.unwrap()))
}

/// Creates a default, empty version of an Aegistrate execution configuration in
/// `~/.config/aegistrate/aegistrate.toml`.
///
/// # Errors
///
/// This function propagates errors from I/O functions called.
#[allow(clippy::missing_panics_doc)]
pub fn create_default_exec_config() -> Aegis<()> {
	let mut config_path = get_preferred_config_path();
	config_path.pop();
	create_dir_all(&config_path)?;

	config_path.push("aegistrate.toml");
	let default_config = ExecConfig::default();
	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(config_path)?;
	aegisize_unit(file.write_all(toml::to_string(&default_config).unwrap().as_bytes()))
}

/// Fetches the execution configuration for Aegistrate, hidden behind a layer of
/// [`OnceCell`].
///
/// # Panics
///
/// This function will panic if the [`GLOBAL_EXEC_CONFIG`] is not initialized.
pub fn get_exec_config<'a>() -> &'a ExecConfig {
	GLOBAL_EXEC_CONFIG.get().unwrap()
}

/// Aegistrate's configuration, used on execution.
///
/// This struct contains **critical** information, never to be logged, nor to be
/// written to anywhere else visible by a third party.
#[must_use]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExecConfig {
	/// The guild ID that this bot functions in.
	pub guild_id: u64,

	/// The bot token used for authentication with the Discord API.
	pub discord_bot_token: String,

	/// The MongoDB server URI to connect to.
	pub mongodb_uri: String,

	/// The amount of time allowed for Aegistrate to start up before exiting
	/// with an error.
	pub timeout_seconds: i64,
}

impl ExecConfig {
	/// Constructs an [`ExecConfig`] object from environment variables.
	///
	/// # Errors
	///
	/// When some environment variables don't load properly, an error may be
	/// returned.
	pub fn from_env() -> Aegis<Self> {
		let _ = dotenv::dotenv();
		Ok(Self {
			guild_id: var("GUILD_ID")?.parse()?,
			discord_bot_token: var("DISCORD_BOT_TOKEN")?,
			mongodb_uri: var("MONGODB_URI")?,
			timeout_seconds: var("TIMEOUT_SECONDS")?.parse()?,
		})
	}

	/// Gets an execution configuration from either **`./aegistrate.toml`
	/// (takes precedence)** or **environment variables**, returning [None] if
	/// neither returned anything or a parsing error happened.
	#[must_use]
	pub fn fetch() -> Option<Self> {
		let config_file = get_exec_config_file();
		let env_config = Self::from_env();
		if let Ok(config_str) = config_file {
			Some(toml::from_str(&config_str).ok()?)
		} else {
			env_config.ok()
		}
	}
}

/// Gets the ID of the guild that this bot is working on.
#[must_use]
pub fn get_working_guild() -> GuildId {
	GuildId(get_exec_config().guild_id)
}

/// Gets the execution config file for Aegistrate, and parses it.
/// The function will only look inside `~/.config/aegistrate/aegistrate.toml`
///
/// # Errors
///
/// The function might fail if [`read_to_string`] fails, or if no Aegistrate
/// execution configuration files were found.
pub fn get_exec_config_file() -> Aegis<String> {
	let path_to_config = get_preferred_config_path();
	if path_to_config.try_exists()? {
		return Ok(read_to_string(path_to_config)?);
	}
	bail!("No Aegistrate execution configuration files were found.")
}

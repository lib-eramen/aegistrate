//! Controls the configuration for the execution of Aegistrate, **NOT** the
//! application's preferences.

use std::{
	env::var,
	fs::{
		read_to_string,
		write,
	},
	path::Path,
};

use anyhow::bail;
use serde::{
	Deserialize,
	Serialize,
};

use crate::aegis::{
	aegisize_unit,
	Aegis,
};

/// The configuration directory for Aegistrate.
pub static CONFIG_DIR: &str = "~/.config/aegistrate";

/// The configuration directory for Aegistrate.
pub static CONFIG_FILE: &str = "~/.config/aegistrate/aegistrate.toml";

/// Aegistrate's configuration, used on execution.
///
/// This struct contains **critical** information, never to be logged, nor to be
/// written to anywhere else visible by a third party.
#[must_use]
#[derive(Clone, Serialize, Deserialize)]
pub struct ExecConfig {
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
		Ok(Self {
			discord_bot_token: var("DISCORD_BOT_TOKEN")?,
			mongodb_uri: var("MONGODB_URI")?,
			timeout_seconds: var("TIMEOUT_SECONDS")?.parse()?,
		})
	}

	/// Gets an execution configuration from either **`~/.config/aegistrate`
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

	/// Writes this configuration object to
	/// `~/.config/aegistrate/aegistrate.toml`.
	///
	/// # Errors
	///
	/// This function might propagate I/O errors from writing to the file
	/// system.
	pub fn write(&self) -> Aegis<()> {
		aegisize_unit(write(CONFIG_FILE, toml::to_string_pretty(self)?.as_bytes()))
	}

	/// Creates an execution configuration object from the provided parameters,
	/// while also writing it to the file system.
	///
	/// # Errors
	///
	/// This function might propagate I/O errors from writing to the file
	/// system.
	pub fn create(
		discord_bot_token: String,
		mongodb_uri: String,
		timeout_seconds: i64,
	) -> Aegis<Self> {
		let config = Self {
			discord_bot_token,
			mongodb_uri,
			timeout_seconds,
		};
		config.write()?;
		Ok(config)
	}
}

/// Gets the execution config file for Aegistrate, and parses it.
/// The function will only look inside the directory for the following file
/// names:
/// - `aegistrate.toml`
/// - `exec-config.toml`
/// - `exec_config.toml`
/// inside of `~/.config/`.
///
/// # Errors
///
/// The function might fail if [`read_to_string`] fails, or if no Aegistrate
/// execution configuration files were found.
pub fn get_exec_config_file() -> Aegis<String> {
	let valid_names = vec!["aegistrate.toml", "exec-config.toml", "exec_config.toml"];
	for filename in valid_names {
		let file = Path::new(CONFIG_DIR).join(filename);
		if file.exists() {
			return Ok(read_to_string(file)?);
		}
	}
	bail!("No Aegistrate execution configuration files were found.")
}

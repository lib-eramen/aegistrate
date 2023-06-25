//! Controls the configuration for the execution of Aegistrate, **NOT** the
//! application's preferences.

use std::{
	env::var,
	fs::read_to_string,
	path::Path,
};

use anyhow::bail;
use log::error;
use serde::{
	Deserialize,
	Serialize,
};
use tokio::sync::OnceCell;

use crate::aegis::{
	aegisize_unit,
	Aegis,
};

/// Aegistrate's global execution configuration.
pub static GLOBAL_EXEC_CONFIG: OnceCell<ExecConfig> = OnceCell::const_new();

/// Initializes the [`EXEC_CONFIG`] static variable.
pub fn initialize_exec_config<'a>() -> Aegis<()> {
	let exec_config = ExecConfig::fetch();
	if exec_config.is_none() {
		error!(
			"No execution configuration files are found for Aegistrate! To get started, create a \
			 .env file in the repo directory, or create one in aegistrate.toml! See the \
			 documentation in docs/EXEC-CONFIG.md for more info."
		);
		bail!("No execution configuration files found.");
	}
	aegisize_unit(GLOBAL_EXEC_CONFIG.set(exec_config.unwrap()))
}

/// Fetches the execution configuration for Aegistrate, hidden behind a layer of
/// [`OnceCell`].
pub fn get_exec_config<'a>() -> &'a ExecConfig {
	GLOBAL_EXEC_CONFIG.get().unwrap()
}

/// Aegistrate's configuration, used on execution.
///
/// This struct contains **critical** information, never to be logged, nor to be
/// written to anywhere else visible by a third party.
#[must_use]
#[derive(Clone, Debug, Serialize, Deserialize)]
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
		let _ = dotenv::dotenv();
		Ok(Self {
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

/// Gets the execution config file for Aegistrate, and parses it.
/// The function will only look inside the repo directory for ``
///
/// # Errors
///
/// The function might fail if [`read_to_string`] fails, or if no Aegistrate
/// execution configuration files were found.
pub fn get_exec_config_file() -> Aegis<String> {
	let path_to_config = Path::new("./aegistrate.toml");
	if path_to_config.try_exists()? {
		return Ok(read_to_string(path_to_config)?);
	}
	bail!("No Aegistrate execution configuration files were found.")
}

//! Your Discord guild's aegis and magistrate! Yes, some people get that
//! portmanteau. This project is way too optimized for everyone's use, but
//! please feel free to remix, remove, add, change, build new parts, etc. for
//! your own Aegistrate.

use std::path::PathBuf;

use dirs::home_dir;

use crate::log::config_log4rs;

mod handler;
mod log;

/// Returns the path leading to where Aegistrate should store all of its files.
///
/// # Panics
///
/// On some operating platforms without a definition for a home directory,
/// this function will panic, since the directory depends on it.
pub fn get_aegistrate_dir_path() -> PathBuf {
	home_dir()
		.expect("Failed to get a home directory")
		.join(".aegistrate/")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	config_log4rs()?;
	Ok(())
}

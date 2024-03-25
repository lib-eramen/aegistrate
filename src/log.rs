//! Configures logging and logging-related actions for Aegistrate.

use std::{
	fs::create_dir_all,
	path::PathBuf,
	sync::OnceLock,
};

use anyhow::anyhow;
use log::LevelFilter;
use log4rs::{
	append::{
		console::{
			ConsoleAppender,
			Target,
		},
		rolling_file::{
			policy::compound::{
				roll::fixed_window::FixedWindowRoller,
				trigger::size::SizeTrigger,
				CompoundPolicy,
			},
			RollingFileAppender,
		},
	},
	config::{
		Appender,
		Root,
	},
	encode::pattern::PatternEncoder,
	filter::threshold::ThresholdFilter,
	Config,
};

use crate::get_aegistrate_dir_path;

/// Maximum log file size before rotation - 10MiB
const TRIGGER_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Number of log files to keep - 20
const LOG_FILE_COUNT: u32 = 20;

/// Name pattern for log files.
const ARCHIVE_DIRECTORY: &str = "log/archive/{}.log";

/// Logging pattern. Includes date+time, then level, module, and message + a new
/// line.
const LOGGING_PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} {l} {t} - {m}{n}";

/// Logging handle provided by `[log4rs]`.
pub static LOG_HANDLE: OnceLock<log4rs::Handle> = OnceLock::new();

/// Creates folders/files for logging, if it already hasn't been created.
fn create_logging_dirs() -> anyhow::Result<()> {
	Ok(create_dir_all(
		get_aegistrate_dir_path().join(ARCHIVE_DIRECTORY),
	)?)
}

/// Configures `[log4rs]` for Aegistrate.
pub fn config_log4rs() -> anyhow::Result<()> {
	create_logging_dirs()?;

	let stdout = ConsoleAppender::builder().target(Target::Stdout).build();
	let policy = CompoundPolicy::new(
		Box::new(SizeTrigger::new(TRIGGER_FILE_SIZE)),
		Box::new(FixedWindowRoller::builder().build(ARCHIVE_DIRECTORY, LOG_FILE_COUNT)?),
	);
	let logfile = RollingFileAppender::builder()
		.encoder(Box::new(PatternEncoder::new(LOGGING_PATTERN)))
		.build(
			get_aegistrate_dir_path().join("aegistrate.log"),
			Box::new(policy),
		)?;

	let config = Config::builder()
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.appender(
			Appender::builder()
				.filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
				.build("stdout", Box::new(stdout)),
		)
		.build(
			Root::builder()
				.appender("logfile")
				.appender("stdout")
				.build(LevelFilter::Trace),
		)?;

	LOG_HANDLE
		.set(log4rs::init_config(config)?)
		.map_err(|_| anyhow!("Something happened while setting LOG_HANDLE."))
}

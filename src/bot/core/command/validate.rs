//! Provides convenience functions for validating a slash command's parameters.

use derive_builder::Builder;
use duration_str::parse_time;
use serenity::model::prelude::application_command::{
	CommandDataOption,
	CommandDataOptionValue,
};
use thiserror::Error;
use time::{
	format_description::well_known::Iso8601,
	Date,
	Duration,
};

/// An error enum that indicates what option is not valid.
#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum InvalidOptionError {
	/// The option is not a date.
	#[error("Not a date (invalidated by ISO 8601): {0}")]
	NotDate(String),

	/// The option is not a time duration.
	#[error("Not a time duration (invalidated by duration_str::parse_time): {0}")]
	NotDuration(String),

	/// The option's time duration is too long (>6 months).
	#[error("Time duration is too long (>6 months): {0}")]
	DurationTooLong(String),

	/// The option is not a guild member.
	#[error("Not a guild member in current working guild: {0}")]
	NotGuildMember(u64),
}

/// A metadata struct that specifies which command options names requires
/// validation that Discord does not natively do.
#[derive(Clone, Default, Builder)]
pub struct ValidatedOptions<'a> {
	/// The parameters that are a date.
	#[builder(default)]
	pub dates: Vec<&'a str>,

	/// The parameters that are a time duration.
	#[builder(default)]
	pub durations: Vec<&'a str>,

	/// The parameters that are guild members.
	#[builder(default)]
	pub guild_members: Vec<&'a str>,
}

#[allow(clippy::missing_panics_doc)]
impl ValidatedOptions<'_> {
	/// Returns this struct's builder.
	#[must_use]
	pub fn builder() -> ValidatedOptionsBuilder<'static> {
		ValidatedOptionsBuilder::default()
	}

	/// Validates a command's date options.
	/// Note that the function will not check for option names that are not
	/// present in the slice of options.
	///
	/// # Errors
	///
	/// On the first invalid date, the function will return an error of type
	/// [`InvalidOptionError::NotDate`].
	pub fn validate_dates(&self, options: &[CommandDataOption]) -> Result<(), InvalidOptionError> {
		if self.dates.is_empty() {
			return Ok(());
		};

		let date_options: Vec<&CommandDataOption> = options
			.iter()
			.filter(|option| self.dates.contains(&option.name.as_str()))
			.collect();

		for option in date_options {
			if let Some(CommandDataOptionValue::String(string)) = &option.resolved {
				if Date::parse(string, &Iso8601::PARSING).is_err() {
					return Err(InvalidOptionError::NotDate(string.to_string()));
				}
			} else {
				return Err(InvalidOptionError::NotDate("not a string".to_string()));
			}
		}
		Ok(())
	}

	/// Validates a command's duration options.
	/// Note that the function will not check for option names that are not
	/// present in the slice of options.
	///
	/// # Errors
	///
	/// On the first invalid duration, the function will return an error of type
	/// [`InvalidOptionError::NotDuration`].
	pub fn validate_durations(
		&self,
		options: &[CommandDataOption],
	) -> Result<(), InvalidOptionError> {
		if self.durations.is_empty() {
			return Ok(());
		};

		let duration_options: Vec<&CommandDataOption> = options
			.iter()
			.filter(|option| self.durations.contains(&option.name.as_str()))
			.collect();

		for option in duration_options {
			if let Some(CommandDataOptionValue::String(string)) = &option.resolved {
				let Ok(duration) = parse_time(string) else {
					return Err(InvalidOptionError::NotDuration(string.to_string()));
				};
				let one_month = 28 * 24 * 60 * 60;
				if duration > Duration::days(one_month) {
					return Err(InvalidOptionError::DurationTooLong(string.to_string()));
				}
			} else {
				return Err(InvalidOptionError::NotDuration("not a string".to_string()));
			}
		}
		Ok(())
	}

	/// Validates a command's guild member options.
	/// Note that the function will not check for options that are not present
	/// in the slice of options.
	///
	/// # Errors
	///
	/// On the first invalid guild member, the function will return an error of
	/// type [`InvalidOptionError::NotGuildMember`].
	pub fn validate_guild_members(
		&self,
		options: &[CommandDataOption],
	) -> Result<(), InvalidOptionError> {
		if self.guild_members.is_empty() {
			return Ok(());
		};

		let guild_member_options: Vec<&CommandDataOption> = options
			.iter()
			.filter(|option| self.guild_members.contains(&option.name.as_str()))
			.collect();

		for option in guild_member_options {
			if let Some(CommandDataOptionValue::User(user, None)) = &option.resolved {
				return Err(InvalidOptionError::NotGuildMember(user.id.into()));
			}
		}
		Ok(())
	}

	/// Validates a command's data options.
	///
	/// # Errors
	///
	/// This function will return an error when any of the options are invalid.
	pub fn validate(&self, options: &[CommandDataOption]) -> Result<(), InvalidOptionError> {
		self.validate_dates(options)?;
		self.validate_durations(options)?;
		self.validate_guild_members(options)?;
		Ok(())
	}
}

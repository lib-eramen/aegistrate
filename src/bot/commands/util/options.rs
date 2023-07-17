//! Contains some functions for working with the [Option]-laden slash command
//! options API.

#![allow(clippy::needless_pass_by_value)]

use duration_str::parse_time;
use serenity::model::prelude::{
	interaction::application_command::{
		CommandDataOption,
		CommandDataOptionValue,
	},
	PartialChannel,
	PartialMember,
	User,
};
use time::{
	format_description::well_known::Iso8601,
	Date,
	Duration,
};

/// Gets an option from an option list.
///
/// # Panics
///
/// If the option found was unable to be resolved (I still don't know what
/// option resolution might be), unwrapping it might cause the
/// function to panic.
#[must_use]
pub fn get_option(options: &[CommandDataOption], name: &str) -> Option<CommandDataOptionValue> {
	options
		.iter()
		.find(|option| option.name.as_str() == name)
		.map(|option| option.resolved.as_ref().unwrap().clone())
}

/// Gets an option from an option list.
///
/// # Panics
///
/// This function should only be called with the knowledge that the option with
/// the provided name is required, as it is unwrapped from [`get_option`].
#[must_use]
pub fn get_required_option(options: &[CommandDataOption], name: &str) -> CommandDataOptionValue {
	get_option(options, name).unwrap()
}

/// Gets an option from an option list, falling back to a default value.
///
/// # Panics
///
/// If the provided `default` option's enum variant is mismatched with the
/// retrieved option, the function will panic.
#[must_use]
pub fn get_option_or(
	options: &[CommandDataOption],
	name: &str,
	default: CommandDataOptionValue,
) -> CommandDataOptionValue {
	use std::mem::discriminant;
	if let Some(option) = get_option(options, name) {
		assert_eq!(
			discriminant(&option),
			discriminant(&default),
			"Option enum variant mismatch: {option:#?} and {default:#?} are not the same",
		);
		option
	} else {
		default
	}
}

/// Gets a string option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::String`] variant.
#[must_use]
pub fn get_string(option: CommandDataOptionValue) -> String {
	if let CommandDataOptionValue::String(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not a string");
}

/// Gets a duration from a string option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::String`] variant, or if parsing the string
/// argument fails.
#[must_use]
pub fn get_duration(option: CommandDataOptionValue) -> Duration {
	if let CommandDataOptionValue::String(value) = option {
		return parse_time(value).unwrap();
	}
	panic!("Wrong type of option: {option:#?} is not a string");
}

/// Gets a date from a string option.
///
/// Note that the only format accepted is `YYYY-MM-DD` specified by
/// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::String`] variant, or if parsing the string
/// argument fails.
#[must_use]
pub fn get_date(option: CommandDataOptionValue) -> Date {
	if let CommandDataOptionValue::String(ref value) = option {
		return Date::parse(value, &Iso8601::PARSING).unwrap();
	}
	panic!("Wrong type of option: {option:#?} is not a string");
}

/// Gets an [`i64`] option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::Integer`] variant.
#[must_use]
pub fn get_int(option: CommandDataOptionValue) -> i64 {
	if let CommandDataOptionValue::Integer(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not an integer");
}

/// Gets an [`f64`] option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::Number`] variant.
#[must_use]
pub fn get_num(option: CommandDataOptionValue) -> f64 {
	if let CommandDataOptionValue::Number(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not an integer");
}

/// Gets a boolean option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::Boolean`] variant.
#[must_use]
pub fn get_bool(option: CommandDataOptionValue) -> bool {
	if let CommandDataOptionValue::Boolean(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not a boolean");
}

/// Gets a user, and [Option]ally a [`PartialMember`] object from the option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::User`] variant.
#[must_use]
pub fn get_user(option: CommandDataOptionValue) -> (User, Option<PartialMember>) {
	if let CommandDataOptionValue::User(user, partial_guild_member) = option {
		return (user, partial_guild_member);
	}
	panic!("Wrong type of option: {option:#?} is not a user");
}

/// Gets a channel option.
///
/// # Panics
///
/// The function panics if the option is not of the
/// [`CommandDataOptionValue::Channel`] variant.
#[must_use]
pub fn get_channel(option: CommandDataOptionValue) -> PartialChannel {
	if let CommandDataOptionValue::Channel(channel) = option {
		return channel;
	}
	panic!("Wrong type of option: {option:#?} is not a channel");
}

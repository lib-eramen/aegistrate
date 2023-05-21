#![allow(clippy::needless_pass_by_value)]

use serenity::model::prelude::{
	interaction::application_command::{
		CommandDataOption,
		CommandDataOptionValue,
	},
	PartialChannel,
	PartialMember,
	User,
};

type Options<'a> = &'a [CommandDataOption];

#[must_use]
pub fn get_option(options: Options<'_>, name: &str) -> Option<CommandDataOptionValue> {
	options
		.iter()
		.find(|option| option.name.as_str() == name)
		.map(|option| option.resolved.as_ref().unwrap().clone())
}

#[must_use]
pub fn get_required_option(options: &[CommandDataOption], name: &str) -> CommandDataOptionValue {
	get_option(options, name).unwrap()
}

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

#[must_use]
pub fn get_string(option: CommandDataOptionValue) -> String {
	if let CommandDataOptionValue::String(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not a string");
}

#[must_use]
pub fn get_int(option: CommandDataOptionValue) -> i64 {
	if let CommandDataOptionValue::Integer(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not an integer");
}

#[must_use]
pub fn get_bool(option: CommandDataOptionValue) -> bool {
	if let CommandDataOptionValue::Boolean(value) = option {
		return value;
	}
	panic!("Wrong type of option: {option:#?} is not a boolean");
}

#[must_use]
pub fn get_user(option: CommandDataOptionValue) -> (User, Option<PartialMember>) {
	if let CommandDataOptionValue::User(user, partial_guild_member) = option {
		return (user, partial_guild_member);
	}
	panic!("Wrong type of option: {option:#?} is not a user");
}

#[must_use]
pub fn get_channel(option: CommandDataOptionValue) -> PartialChannel {
	if let CommandDataOptionValue::Channel(channel) = option {
		return channel;
	}
	panic!("Wrong type of option: {option:#?} is not a channel");
}

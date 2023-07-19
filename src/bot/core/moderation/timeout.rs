//! The time out moderation command.

use serenity::{
	http::CacheHttp,
	model::{
		prelude::application_command::ApplicationCommandInteraction,
		Timestamp,
	},
	prelude::Context,
};
use time::OffsetDateTime;

use crate::{
	aegis::Aegis,
	bot::core::moderation::ModerationParameters,
};

/// Times out a member in a guild.
///
/// # Panics
///
/// This function may panic if the command is invoked not within a guild (a.k.a.
/// from the bot's DMs).
///
/// # Errors
///
/// This function propagates errors from
/// [`serenity::builder::edit_member::EditMember::disable_communication_until_datetime`].
pub async fn timeout(
	params: &ModerationParameters,
	context: &Context,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	let timed_out_until = OffsetDateTime::now_utc() + params.duration.unwrap();
	interaction
		.guild_id
		.unwrap()
		.edit_member(context.http(), params.user, |member| {
			member.disable_communication_until_datetime(
				Timestamp::from_unix_timestamp(timed_out_until.unix_timestamp()).unwrap(),
			)
		})
		.await?;
	Ok(())
}

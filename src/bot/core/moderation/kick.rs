//! The kixk moderation command.

use serenity::{
	http::CacheHttp,
	model::prelude::application_command::ApplicationCommandInteraction,
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::core::moderation::ModerationParameters,
};

/// Kicks a member from a guild.
///
/// # Panics
///
/// This function may panic if the command is invoked not within a guild (a.k.a.
/// from the bot's DMs).
///
/// # Errors
///
/// This function propagates errors from
/// [`serenity::model::guild::guild_id::GuildId::kick_with_reason`].
pub async fn kick(
	params: &ModerationParameters,
	context: &Context,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	interaction
		.guild_id
		.unwrap()
		.kick_with_reason(context.http(), params.user, &params.reason)
		.await?;
	Ok(())
}

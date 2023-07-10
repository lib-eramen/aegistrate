//! Contains functions that make working with the message-sending portion of
//! Discord's API a bit easier to.

#![allow(clippy::option_option)]

use serenity::{
	builder::CreateEmbed,
	http::Http,
	model::prelude::{
		interaction::{
			application_command::ApplicationCommandInteraction,
			InteractionResponseType::ChannelMessageWithSource,
		},
		Message,
		MessageId,
	},
};

use crate::{
	aegis::{
		aegisize,
		Aegis,
	},
	bot::commands::components::embed::create_info_embed,
};

/// Controlling Discord message response options.
#[derive(Clone, Copy)]
pub enum ResponseOptions {
	/// Creates an original interaction response message. The contained `bool`
	/// property controls the ephemerality of the message.
	CreateOrignial(bool),

	/// Edits the orignial interaction response message.
	EditOriginal,

	/// Creates a followup message. The contained `bool` property controls the
	/// ephemerality of the message.
	CreateFollowup(bool),

	/// Edits the message with the provided ID.
	EditFollowup(MessageId),
}

/// Sends a message that notifies the user that they are waiting.
/// These are recommended over setting the `defer` option when sending an
/// interaction response, as just working with a message is far more flexible
/// than having a placeholder animation in place of it. Additionally, with tasks
/// that are multiple stages, one might want to have a "status dialog" embed of
/// sorts.
#[allow(clippy::missing_errors_doc)]
pub async fn wait_a_moment(
	http: &Http,
	interaction: &ApplicationCommandInteraction,
	options: ResponseOptions,
) -> Aegis<Option<Message>> {
	respond_with_embed(http, interaction, options, |embed| {
		create_info_embed(
			embed,
			"Wait a moment...".to_string(),
			"Hang tight, I'm working on it.".to_string(),
		)
	})
	.await
}

/// Responds to the interaction with an embed.
#[allow(clippy::missing_errors_doc)]
pub async fn respond_with_embed<E>(
	http: &Http,
	interaction: &ApplicationCommandInteraction,
	options: ResponseOptions,
	create_embed: E,
) -> Aegis<Option<Message>>
where
	E: FnMut(&mut CreateEmbed) -> &mut CreateEmbed, {
	match options {
		ResponseOptions::CreateOrignial(ephemeral) => aegisize(
			interaction
				.create_interaction_response(http, |response| {
					response
						.kind(ChannelMessageWithSource)
						.interaction_response_data(|message| {
							message.ephemeral(ephemeral).embed(create_embed)
						})
				})
				.await,
			|_| None,
		),
		ResponseOptions::EditOriginal => aegisize(
			interaction
				.edit_original_interaction_response(http, |message| message.embed(create_embed))
				.await,
			|_| None,
		),
		ResponseOptions::CreateFollowup(ephemeral) => aegisize(
			interaction
				.create_followup_message(http, |message| {
					message.ephemeral(ephemeral).embed(create_embed)
				})
				.await,
			Some,
		),
		ResponseOptions::EditFollowup(message_id) => aegisize(
			interaction
				.edit_followup_message(http, message_id, |message| message.embed(create_embed))
				.await,
			|_| None,
		),
	}
}

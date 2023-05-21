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
	commands::components::embed::create_info_embed,
};

#[derive(Clone, Copy)]
pub struct EmbedResponseOptions {
	pub followup: Option<Option<MessageId>>,
	pub edit_original: bool,
	pub ephemeral: bool,
}

impl EmbedResponseOptions {
	#[must_use]
	pub fn is_followup(&self) -> bool {
		self.followup.is_some()
	}

	#[must_use]
	pub fn is_followup_edit(&self) -> bool {
		self.followup.is_some() && self.followup.unwrap().is_some()
	}

	#[must_use]
	pub fn followup_id(&self) -> MessageId {
		self.followup.unwrap().unwrap()
	}

	#[must_use]
	pub fn interaction_response(ephemeral: bool) -> Self {
		Self {
			followup: None,
			edit_original: false,
			ephemeral,
		}
	}

	#[must_use]
	pub fn interaction_edit() -> Self {
		Self {
			followup: None,
			edit_original: true,
			ephemeral: false,
		}
	}

	#[must_use]
	pub fn follow_up(ephemeral: bool) -> Self {
		Self {
			followup: Some(None),
			edit_original: false,
			ephemeral,
		}
	}

	#[must_use]
	pub fn followup_edit(message_id: Option<MessageId>) -> Self {
		Self {
			followup: Some(message_id),
			edit_original: false,
			ephemeral: false,
		}
	}
}

pub async fn wait_a_moment(
	http: &Http,
	interaction: &ApplicationCommandInteraction,
	options: EmbedResponseOptions,
	custom_message: Option<String>,
) -> Aegis<Option<Message>> {
	respond_with_embed(http, interaction, options, |embed| {
		create_info_embed(
			embed,
			format!(
				"{}...",
				custom_message
					.clone()
					.unwrap_or_else(|| "Wait a moment".to_string())
			),
			"Hang tight, I'm working on it.".to_string(),
		)
		.footer(|footer| {
			footer.text("\"Patience is a virtue\", said no one ever. No one likes waiting!")
		})
	})
	.await
}

pub async fn respond_with_embed<E>(
	http: &Http,
	interaction: &ApplicationCommandInteraction,
	options: EmbedResponseOptions,
	create_embed: E,
) -> Aegis<Option<Message>>
where
	E: FnMut(&mut CreateEmbed) -> &mut CreateEmbed, {
	if options.is_followup() {
		if options.is_followup_edit() {
			aegisize(
				interaction
					.edit_followup_message(http, options.followup_id(), |message| {
						message.embed(create_embed)
					})
					.await,
				|_| None,
			)
		} else {
			aegisize(
				interaction
					.create_followup_message(http, |message| {
						message.ephemeral(options.ephemeral).embed(create_embed)
					})
					.await,
				Some,
			)
		}
	} else if options.edit_original {
		aegisize(
			interaction
				.edit_original_interaction_response(http, |message| message.embed(create_embed))
				.await,
			|_| None,
		)
	} else {
		aegisize(
			interaction
				.create_interaction_response(http, |response| {
					response
						.kind(ChannelMessageWithSource)
						.interaction_response_data(|message| {
							message.ephemeral(options.ephemeral).embed(create_embed)
						})
				})
				.await,
			|_| None,
		)
	}
}

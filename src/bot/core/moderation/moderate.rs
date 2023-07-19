//! A general implementation for a moderate command that can be used for any
//! kind of moderation action. Use [moderate] alongside the API provided by
//! [`crate::bot::core::moderation`] to create a moderation command in a robust
//! way.

use anyhow::bail;
use serenity::{
	http::{
		CacheHttp,
		Http,
	},
	model::prelude::{
		application_command::ApplicationCommandInteraction,
		UserId,
	},
	prelude::{
		Context,
		Mentionable,
	},
};

use crate::{
	aegis::{
		aegisize_unit,
		Aegis,
	},
	bot::{
		commands::{
			components::embed::{
				create_error_embed,
				create_info_embed,
				create_warning_embed,
			},
			util::message::{
				respond_with_embed,
				wait_a_moment,
				ResponseOptions,
			},
		},
		core::moderation::{
			ban::ban,
			capitalize_first,
			kick::kick,
			timeout::timeout,
			ModerationAction,
			ModerationEligibility,
			ModerationParameters,
		},
	},
	exec_config::get_working_guild_id,
};

/// Notifies a guild member of a moderation action being performed on them.
///
/// # Panics
///
/// Panics if the guild ID does not correspond to a valid guild that the cache
/// can find.
///
/// # Errors
///
/// This function propagates errors from [`serenity::model::user::User::dm`].
async fn notify_moderated_member(
	action: ModerationAction,
	params: &ModerationParameters,
	context: &Context,
) -> Aegis<()> {
	let http = context.http();
	let guild = get_working_guild_id();
	let user = guild.member(http, params.user).await?.user;

	let embed_description = format!(
		"You have been {} by guild {}.",
		action.get_moderation_verb_past(),
		guild.name(context.cache().unwrap()).unwrap()
	);

	aegisize_unit(
		user.dm(http, |message| {
			message.embed(|embed| {
				create_info_embed(
					embed,
					format!("{}!", capitalize_first(action.get_moderation_verb_past())),
					embed_description,
				)
			})
		})
		.await,
	)
}

/// Sends an error embed to the channel the moderation action was performed
/// for being unable to assess the eligibility of the member for moderation.
async fn send_eligibility_assessment_failed_embed(
	error_message: String,
	http: &Http,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
		create_error_embed(
			embed,
			"Failed to assess eligibility!",
			format!(
				"The bot failed to assess the eligibility of the member for moderation: \
				 {error_message}. To be safe, the bot will abort whatever moderation action that \
				 is being performed right now."
			),
		)
	})
	.await
	.map(|_| ())
}

/// Sends an embed to the channel the moderation action was performed in, with
/// the result from notifying the member of the moderation action.
///
/// # Errors
///
/// This function propagates errors from
/// [`serenity::model::application::interaction::application_command::ApplicationCommandInteraction::create_followup_message`].
async fn send_notification_results(
	dm_result: Aegis<()>,
	member: UserId,
	http: &Http,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	let username = member.mention();
	respond_with_embed(
		http,
		interaction,
		ResponseOptions::CreateFollowup(false),
		|embed| {
			if let Err(ref why) = dm_result {
				create_warning_embed(
					embed,
					"Notification failed!",
					format!("The bot failed to send {username} a DM. Please notify them manually."),
				)
				.field("Failure reason", why, false)
			} else {
				create_info_embed(embed, "Notified!", format!("{username} has been notified."))
			}
		},
	)
	.await
	.map(|_| ())
}

/// Sends an embed to the channel the moderation action was performed in, with
/// the result of the moderation action.
///
/// # Errors
///
/// This function propagates errors from
/// [`serenity::model::application::interaction::application_command::ApplicationCommandInteraction::edit_interaction_response`].
async fn send_action_results(
	result: Aegis<()>,
	action: ModerationAction,
	params: &ModerationParameters,
	http: &Http,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	let username = params.user.mention();
	respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
		if let Err(ref why) = result {
			create_error_embed(
				embed,
				format!(
					"The bot failed to {} {username}.",
					action.get_moderation_verb(),
				),
				why,
			)
		} else {
			let duration_text = params
				.duration
				.map_or("Not applicable".to_string(), |duration| {
					duration.to_string()
				});

			create_info_embed(
				embed,
				"Success!",
				format!(
					"{username} has been successfully {}.",
					action.get_moderation_verb_past()
				),
			)
			.field(
				"Action",
				capitalize_first(action.get_moderation_verb()),
				true,
			)
			.field("Member", username, true)
			.field("Duration", duration_text, true)
			.field("Reason", &params.reason, false)
		}
	})
	.await
	.map(|_| ())
}

/// Calls the moderation action function matching the current enum.
///
/// This function also notifies the member of the moderation action being
/// performed on them.
///
/// # Errors
///
/// This function will return an error if the bot fails to send a message to the
/// channel, or if the bot fails to perform the moderation action.
#[allow(clippy::missing_panics_doc)]
pub async fn moderate(
	action: ModerationAction,
	params: &ModerationParameters,
	context: &Context,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	let http = context.http();
	wait_a_moment(
		http,
		interaction,
		ResponseOptions::CreateOrignial(false),
		None,
	)
	.await?;

	let eligibility = ModerationEligibility::assess(
		interaction.user.id,
		params.user,
		http,
		context.cache().unwrap(),
	)
	.await;

	if let Err(why) = eligibility {
		send_eligibility_assessment_failed_embed(why.to_string(), http, interaction).await?;
		bail!("Failed to assess moderation eligibility: `{why}`");
	}
	// When let-chains will be stabilized is a question lost to time
	else if let Ok(eligibility) = eligibility {
		if eligibility != ModerationEligibility::Eligible {
			return respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
				eligibility.create_ineligibility_embed(
					interaction.user.mention(),
					params.user.mention(),
					action,
					embed,
				)
			})
			.await
			.map(|_| ());
		}
	}

	send_notification_results(
		notify_moderated_member(action, params, context).await,
		params.user,
		http,
		interaction,
	)
	.await?;

	let action_result = match action {
		ModerationAction::Ban => ban(params, context, interaction).await,
		ModerationAction::Kick => kick(params, context, interaction).await,
		ModerationAction::Timeout => timeout(params, context, interaction).await,
	};
	send_action_results(action_result, action, params, http, interaction).await
}

//! Contains functions that represent moderation actions in Aegistrate and a
//! common API for supplying it with parameters.
//!
//! This module is designed for implementing moderation commands that are as
//! simple as calling the right API function, while having advanced handling and
//! validation already available.
//!
//! Every moderation command function in this API conforms to the following
//! signature:
//! ```
//! pub async fn name(
//!     params: &ModerationParameters,
//!     context: &serenity::prelude::Context,
//!     interaction:
//! &serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction,
//!
//! ) -> Aegis<()>
//! ```
//! which then you can call directly or use via a more global function that does
//! more advanced handling: [`moderate::moderate`]

use anyhow::bail;
use derive_builder::Builder;
use serenity::{
	builder::CreateEmbed,
	client::Cache,
	http::{
		CacheHttp,
		Http,
	},
	model::prelude::{
		Mention,
		UserId,
	},
};
use time::Duration;

use crate::{
	aegis::Aegis,
	bot::commands::components::embed::create_error_embed,
	exec_config::get_working_guild_id,
};

pub mod ban;
pub mod kick;
pub mod moderate;
pub mod timeout;

fn capitalize_first(s: &str) -> String {
	let mut chars = s.chars();
	match chars.next() {
		None => String::new(),
		Some(c) => c.to_uppercase().chain(chars).collect(),
	}
}

/// Represents parameters used by every moderation action in Aegistrate.
#[derive(Clone, Default, Builder)]
#[must_use]
pub struct ModerationParameters {
	/// The member to perform the moderation action on.
	pub user: UserId,

	/// The reason to perform this moderation action.
	pub reason: String,

	/// The ([Option]al) duration to perform this moderation action for.
	/// Some moderation actions use this parameter, some does not.
	pub duration: Option<Duration>,
}

impl ModerationParameters {
	/// Returns the builder for this struct.
	#[must_use]
	pub fn builder() -> ModerationParametersBuilder {
		ModerationParametersBuilder::default()
	}
}

/// Represents all of the current moderation actions.
#[derive(Clone, Copy)]
// TODO: Add as we go
#[must_use]
pub enum ModerationAction {
	/// Bans a member from the guild.
	Ban,

	/// Kicks a member from the guild.
	Kick,

	/// Times out a member from the guild.
	Timeout,
}

impl ModerationAction {
	/// Returns whether this action requires a duration to be specified.
	#[must_use]
	pub fn requires_duration(self) -> bool {
		matches!(self, Self::Timeout)
	}
}

/// Represents possibilities between moderators and those being moderated.
#[derive(Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum ModerationEligibility {
	/// The member is eligible for moderation.
	Eligible,

	/// The "member" is a bot user.
	BotUser,

	/// The member is the moderator themselves.
	ServerOwner,

	/// The member is the bot itself.
	AreThemselves,

	/// The member is higher in the hierarchy than the moderator.
	LowerHierarchy,
}

impl ModerationEligibility {
	/// Returns the eligibility of a member for moderation.
	///
	/// # Errors
	///
	/// This function will return an error if the bot fails to get the guild
	/// from the cache.
	pub async fn assess(
		moderator: UserId,
		member: UserId,
		http: impl AsRef<Http> + CacheHttp,
		cache: impl AsRef<Cache>,
	) -> Aegis<Self> {
		let Some(guild) = get_working_guild_id().to_guild_cached(&cache) else {
			bail!("Failed to get guild data from the cache.")
		};

		Ok(if moderator == member {
			Self::AreThemselves
		} else if member.to_user(http).await?.bot {
			Self::BotUser
		} else if moderator != guild.owner_id {
			Self::ServerOwner
		} else if guild
			.greater_member_hierarchy(cache, moderator, member)
			.is_some_and(|greater_member| greater_member == member)
		{
			Self::LowerHierarchy
		} else {
			Self::Eligible
		})
	}

	/// Creates and returns an error embed to the channel the moderation action
	/// was performed detailing the reason why the member is not eligible for
	/// moderation.
	fn create_ineligibility_embed(
		self,
		moderator_mention: Mention,
		member_mention: Mention,
		action: ModerationAction,
		embed: &mut CreateEmbed,
	) -> &mut CreateEmbed {
		let (verb_present, verb_past) = (
			action.get_moderation_verb(),
			action.get_moderation_verb_past(),
		);
		let error = match self {
			Self::Eligible => panic!("This case should not happen!"),
			Self::BotUser => {
				format!(
					"{member_mention} is a bot user, and cannot be {verb_past}. Please manage the \
					 integration attached to {member_mention} instead.",
				)
			},
			Self::ServerOwner => {
				format!("{member_mention} is a server owner, and cannot be {verb_past}.")
			},
			Self::AreThemselves => {
				format!("You cannot {verb_present} yourself, {moderator_mention}.")
			},
			Self::LowerHierarchy => {
				format!(
					"{member_mention} is higher in the hierarchy than {moderator_mention}, and \
					 cannot be {verb_past}.",
				)
			},
		};
		let cause = match self {
			ModerationEligibility::Eligible => panic!("This case should not happen!"),
			ModerationEligibility::BotUser => {
				format!(
					"Bot users are attached to integrations, so it is better to manage them \
					 directly than to {verb_present} the bot users."
				)
			},
			ModerationEligibility::ServerOwner => {
				format!(
					"Server owners are the highest in the hierarchy, so they cannot be \
					 {verb_past}."
				)
			},
			ModerationEligibility::AreThemselves => {
				"You don't have higher permissions than yourself.".to_string()
			},
			ModerationEligibility::LowerHierarchy => {
				format!(
					"You don't have higher permissions than the member you are trying to \
					 {verb_past}."
				)
			},
		};
		create_error_embed(embed, error, cause)
	}
}

impl ModerationAction {
	/// Gets the verb used to describe this moderation action, present tense.
	#[must_use]
	pub fn get_moderation_verb(self) -> &'static str {
		match self {
			Self::Ban => "ban",
			Self::Kick => "kick",
			Self::Timeout => "timeout",
		}
	}

	/// Gets the verb used to describe this moderation action, past tense.
	#[must_use]
	pub fn get_moderation_verb_past(self) -> &'static str {
		match self {
			Self::Ban => "banned",
			Self::Kick => "kicked",
			Self::Timeout => "timed out",
		}
	}
}

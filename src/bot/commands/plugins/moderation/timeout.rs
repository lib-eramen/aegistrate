//! Times out a member in Discord. See also [Timeout].

use async_trait::async_trait;
use duration_str::parse_time;
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::{
		application_command::{
			ApplicationCommandInteraction,
			CommandDataOptionValue,
		},
		command::CommandOptionType,
	},
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::{
		commands::util::options::{
			get_option_or,
			get_required_option,
			get_string,
			get_user,
		},
		core::{
			command::{
				validate::ValidatedOptions,
				Command,
				Metadata,
			},
			moderation::{
				moderate::moderate,
				ModerationAction,
				ModerationParameters,
			},
			plugin::Plugin,
		},
	},
};

/// The unit struct containing the implementation for the `/timeout`
/// command.
pub struct Timeout;

#[async_trait]
impl Command for Timeout {
	fn metadata(&self) -> Metadata<'_> {
		Metadata::builder()
			.name("timeout")
			.description("Times out a member in the guild.")
			.plugin(Plugin::Moderation)
			.cooldown_secs(5)
			.validated_options(
				ValidatedOptions::builder()
					.guild_members(vec!["member"])
					.durations(vec!["duration"])
					.build()
					.unwrap(),
			)
			.build()
			.unwrap()
	}

	fn register<'a>(
		&self,
		command: &'a mut CreateApplicationCommand,
	) -> &'a mut CreateApplicationCommand {
		command
			.create_option(|member| {
				member
					.name("member")
					.description("The member to time out in the guild.")
					.kind(CommandOptionType::User)
					.required(true)
			})
			.create_option(|duration| {
				duration
					.name("duration")
					.description("The duration of the timeout.")
					.kind(CommandOptionType::String)
					.required(true)
			})
			.create_option(|reason| {
				reason
					.name("reason")
					.description("The reason to time out this member.")
					.kind(CommandOptionType::String)
					.required(false)
			})
	}

	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()> {
		let options = &interaction.data.options;
		let moderation_params = ModerationParameters::builder()
			.user(get_user(get_required_option(options, "member")).0.id)
			.reason(get_string(get_option_or(
				options,
				"reason",
				CommandDataOptionValue::String("No reason provided.".to_string()),
			)))
			.duration(Some(
				parse_time(get_string(get_required_option(options, "duration"))).unwrap(),
			))
			.build()
			.unwrap();
		moderate(
			ModerationAction::Timeout,
			&moderation_params,
			context,
			interaction,
		)
		.await
	}
}

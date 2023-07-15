//! Kicks a member in Discord. See also [Kick].

use async_trait::async_trait;
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

/// The unit struct containing the implementation for the `/kick`
/// command.
pub struct Kick;

#[async_trait]
impl Command for Kick {
	fn metadata(&self) -> Metadata<'_> {
		Metadata::builder()
			.name("kick")
			.description("Kicks a member from the guild.")
			.plugin(Plugin::Moderation)
			.cooldown_secs(5)
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
					.description("The member to kick from the guild.")
					.kind(CommandOptionType::User)
					.required(true)
			})
			.create_option(|reason| {
				reason
					.name("reason")
					.description("The reason to kick this member.")
					.kind(CommandOptionType::String)
					.min_length(3)
					.max_length(100)
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
			.duration(None)
			.build()
			.unwrap();
		moderate(
			ModerationAction::Kick,
			&moderation_params,
			context,
			interaction,
		)
		.await
	}
}

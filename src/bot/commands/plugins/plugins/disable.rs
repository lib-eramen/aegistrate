//! Disables a plugin for a guild (if not already disabled.) See also
//! [`DisablePlugin`].

use async_trait::async_trait;
use serenity::{
	builder::CreateApplicationCommand,
	http::CacheHttp,
	model::prelude::{
		command::CommandOptionType,
		interaction::application_command::ApplicationCommandInteraction,
	},
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::{
		commands::{
			components::embed::{
				create_error_embed,
				create_success_embed,
			},
			util::{
				message::{
					respond_with_embed,
					wait_a_moment,
					ResponseOptions,
				},
				options::{
					get_required_option,
					get_string,
				},
			},
		},
		core::{
			command::{
				Command,
				Metadata,
			},
			plugin::{
				disable_plugin,
				Plugin,
			},
		},
	},
};

/// The unit struct containing the implementation for the `/disable`
/// command.
pub struct Disable;

#[async_trait]
impl Command for Disable {
	fn metadata(&self) -> Metadata<'_> {
		Metadata::builder()
			.name("disable")
			.description("Disables a plugin for the current guild.")
			.plugin(Plugin::Plugins)
			.cooldown_secs(10)
			.aliases(None)
			.build()
			.unwrap()
	}

	fn register<'a>(
		&self,
		command: &'a mut CreateApplicationCommand,
	) -> &'a mut CreateApplicationCommand {
		command.create_option(|plugin| {
			for plugin_name in Plugin::non_default_plugins()
				.into_iter()
				.map(Plugin::to_name)
			{
				plugin.add_string_choice(plugin_name, plugin_name);
			}
			plugin
				.name("plugin")
				.description("The plugin to disable for the current guild.")
				.kind(CommandOptionType::String)
				.required(true)
		})
	}

	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()> {
		let http = context.http();
		wait_a_moment(
			context.http(),
			interaction,
			ResponseOptions::CreateOrignial(false),
		)
		.await?;

		let plugin = Plugin::from_name(&get_string(get_required_option(
			&interaction.data.options,
			"plugin",
		)))
		.unwrap();

		if let Err(why) = disable_plugin(plugin, context).await {
			respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
				create_error_embed(
					embed,
					format!("An error happened: `{why}`"),
					format!(
						"The plugin `{}` might have been already disabled, or a networking error \
						 has happened.",
						plugin.to_name()
					),
					None,
				)
			})
			.await
			.map(|_| ())
		} else {
			respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
				create_success_embed(
					embed,
					format!("Plugin {} disabled!", plugin.to_name()),
					format!(
						"Successfully disabled plugin {}! Commands that were removed from your \
						 guild were: {}",
						plugin.to_name(),
						enabled_commands_string(plugin)
					),
				)
			})
			.await?;
			Ok(())
		}
	}
}

/// Gets a comma-and-space-delimited list of commands enabled by a plugin.
#[must_use]
fn enabled_commands_string(plugin: Plugin) -> String {
	plugin
		.get_commands()
		.iter()
		.map(|command| command.metadata().name)
		.map(|name| format!("`/{name})`"))
		.collect::<Vec<String>>()
		.join(", ")
}

//! Enables a plugin for a guild (if not already enabled.) See also
//! [`EnablePlugin`].

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
	commands::{
		components::embed::{
			create_error_embed,
			create_info_embed,
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
			enable_plugin,
			Plugin,
		},
	},
};

/// The unit struct containing the implementation for the `/enable`
/// command.
pub struct Enable;

#[async_trait]
impl Command for Enable {
	fn metadata(&self) -> Metadata<'_> {
		Metadata::builder()
			.name("enable")
			.description("Enables a plugin for the current guild.")
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
				.description("The plugin to enable for the current guild.")
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

		if plugin.requires_setup() {
			return respond_with_embed(
				http,
				interaction,
				ResponseOptions::CreateOrignial(false),
				|embed| {
					create_info_embed(
						embed,
						"Dashboard setup required",
						"This plugin requires further setup with the dashboard UI.",
					)
				},
			)
			.await
			.map(|_| ());
		}

		if let Err(why) = enable_plugin(plugin, http).await {
			respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
				create_error_embed(
					embed,
					format!("An error happened: `{why}`"),
					format!(
						"The plugin `{}` might have been already enabled, setup steps weren't \
						 completed, or a networking error has happened.",
						plugin.to_name()
					),
					None,
				)
			})
			.await
		} else {
			respond_with_embed(http, interaction, ResponseOptions::EditOriginal, |embed| {
				create_success_embed(
					embed,
					format!("Plugin {} enabled!", plugin.to_name()),
					format!(
						"Successfully enabled plugin {}! Commands that were enabled for your \
						 guild were: {}",
						plugin.to_name(),
						enabled_commands_string(plugin)
					),
				)
			})
			.await
		}
		.map(|_| ())
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

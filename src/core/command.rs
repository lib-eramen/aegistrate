use async_trait::async_trait;
use derive_builder::Builder;
use serenity::{
	builder::CreateApplicationCommand,
	model::prelude::interaction::application_command::ApplicationCommandInteraction,
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	core::plugin::Plugin,
};

pub type Commands = Vec<Box<dyn Command>>;

#[derive(Copy, Clone, Builder)]
pub struct Metadata<'a> {
	pub name: &'a str,
	pub plugin: Plugin,
	pub cooldown_secs: u64,
}

#[async_trait]
pub trait Command {
	fn metadata(&self) -> Metadata<'_>;
	fn register<'a>(
		&self,
		command: &'a mut CreateApplicationCommand,
	) -> &'a mut CreateApplicationCommand;
	async fn execute(
		&self,
		context: &Context,
		interaction: &ApplicationCommandInteraction,
	) -> Aegis<()>;
}

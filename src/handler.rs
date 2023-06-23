//! Contains an event handler struct that complies with [serenity]'s
//! [`EventHandler`] trait API, and also some helper function that controls
//! Aegistrate's state.

use std::{
	env::var,
	sync::atomic::{
		AtomicBool,
		Ordering,
	},
	thread,
	time::Duration,
};

use async_trait::async_trait;
use log::{
	error,
	info,
};
use pluralizer::pluralize;
use serenity::{
	client::Cache,
	http::{
		CacheHttp,
		Http,
	},
	model::{
		prelude::{
			interaction::{
				application_command::ApplicationCommandInteraction,
				Interaction,
			},
			Activity,
			GuildId,
			Ready,
			UnavailableGuild,
		},
		user::{
			CurrentUser,
			OnlineStatus,
		},
	},
	prelude::{
		Context,
		EventHandler,
	},
};
use tokio::{
	sync::OnceCell,
	time::sleep,
};

use crate::{
	aegis::Aegis,
	commands::{
		components::embed::create_error_embed,
		util::message::{
			respond_with_embed,
			ResponseOptions,
		},
	},
	core::{
		command::{
			command_by_name,
			Command,
		},
		cooldown::{
			cooled_down,
			get_remaining_cooldown,
			use_last,
		},
		plugin::get_guild_commands,
	},
	data::init_all_data,
};

/// Spawns a timeout checker that exits the program if [`DISCORD_READY`] is not
/// set to `true` after an environment-specified number of seconds.
pub fn spawn_timeout_checker(ready_up_time: u64) {
	thread::sleep(Duration::from_secs(ready_up_time));
	if !DISCORD_READY.load(Ordering::Relaxed) {
		error!("Services not ready for {ready_up_time} seconds");
		std::process::exit(1);
	}
}

/// Unit struct that implements [`EventHandler`]. Is Aegistrate's core Discord
/// event handler.
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, bot_data: Ready) {
		set_up_database().await.unwrap_or_else(|why| {
			panic!("Database failed to set up: {why}");
		});
		if !DISCORD_READY.load(Ordering::Relaxed) {
			Self::discord_ready_up(&context, &bot_data).await;
			Self::initialize_systems(&context, &bot_data)
				.await
				.unwrap_or_else(|why| error!("Initializing system failed: {why}"));
		}
		Self::appear_active(&context, &bot_data).await;
	}

	async fn interaction_create(&self, context: Context, interaction: Interaction) {
		match interaction {
			Interaction::ApplicationCommand(ref app_interaction) => {
				Self::run_application_command(&context, app_interaction).await;
			},
			_ => return,
		};
	}
}

impl Handler {
	/// Readies up the Discord service portion of Aegistrate.
	///
	/// # Panics
	///
	/// This function may panic if [`AEGISTRATE_USER`] is not successfully set
	/// to the provided user.
	pub async fn discord_ready_up(context: &Context, bot_data: &Ready) {
		AEGISTRATE_USER.set(bot_data.user.clone()).unwrap();
		DISCORD_READY.store(true, Ordering::Relaxed);
		context
			.set_presence(
				Some(Activity::playing("the waiting game...")),
				OnlineStatus::DoNotDisturb,
			)
			.await;
	}

	/// Appear active to both user and developer ends.
	async fn appear_active(context: &Context, bot_data: &Ready) {
		context
			.set_presence(
				Some(Activity::watching("over the server")),
				OnlineStatus::Online,
			)
			.await;
		READY_TO_GO.store(true, Ordering::Relaxed);
		info!(
			"{} has reached the Out-Post!\nUser: {:#?}",
			bot_data.user.tag(),
			get_aegistrate_user()
		);
	}

	/// Registers multiple commands to a guild.
	#[allow(clippy::cast_possible_wrap)]
	async fn register_commands(
		http: &Http,
		cache: &Cache,
		guild: GuildId,
		commands: Vec<Box<dyn Command>>,
	) -> Aegis<()> {
		let commands_count = commands.len();
		for command in commands {
			command.register_to_guild(http, cache, guild).await?;
			sleep(Duration::from_secs_f32(REGISTER_COMMAND_INTERVAL)).await;
		}
		info!(
			"Guild \"{}\" ({}) registered {}",
			guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
			guild.0,
			pluralize("command", commands_count as isize, true),
		);
		Ok(())
	}

	/// Handles command registration for a guild, using the commands from the
	/// guild's enabled plugins.
	async fn set_up_commands(context: &Context, guild: &UnavailableGuild) -> Aegis<()> {
		guild
			.id
			.set_application_commands(context.http(), |commands| commands)
			.await?;
		let guild_commands = get_guild_commands(guild.id.into()).await;
		Self::register_commands(
			context.http(),
			context.cache().unwrap(),
			guild.id,
			guild_commands,
		)
		.await
	}

	/// Initializes all systems for Aegistrate.
	async fn initialize_systems(context: &Context, bot_data: &Ready) -> Aegis<()> {
		for guild in &bot_data.guilds {
			init_all_data(guild.id.into()).await?;
			Self::set_up_commands(context, guild).await?;
		}
		Ok(())
	}

	/// Responds to the interaction with a not-ready message.
	async fn not_ready_respond(
		http: &Http,
		app_interaction: &ApplicationCommandInteraction,
	) -> Aegis<()> {
		respond_with_embed(
			http,
			app_interaction,
			ResponseOptions::CreateOrignial(true),
			|embed| {
				create_error_embed(
					embed,
					"Rude! I'm not even done getting ready!",
					"Aegistrate hasn't finished all of its ready-up procedures. Coffee takes time \
					 to brew, y'know!",
					Some(
						"Give Aegistrate a bit of time - you should see its status turn from Do \
						 Not Disturb to Online soon!",
					),
				)
			},
		)
		.await
		.map(|_| ())
	}

	/// Responds to the interaction with an unknown-command message.
	async fn unknown_command_respond(
		http: &Http,
		app_interaction: &ApplicationCommandInteraction,
		command_name: &str,
	) -> Aegis<()> {
		respond_with_embed(
			http,
			app_interaction,
			ResponseOptions::CreateOrignial(true),
			|embed| {
				create_error_embed(
					embed,
					format!(
						"Sorry champ, can't help you out there. I dunno what a \
						 \"/{command_name}\" is."
					),
					format!("Failed to get a command with the name \"/{command_name}\"."),
					None,
				)
			},
		)
		.await
		.map(|_| ())
	}

	/// Responds to the interaction with a not-cooled-down message.
	async fn not_cooled_down_respond(
		http: &Http,
		app_interaction: &ApplicationCommandInteraction,
		command: &dyn Command,
	) -> Aegis<()> {
		let remaining_cooldown = get_remaining_cooldown(
			app_interaction.guild_id.unwrap().into(),
			app_interaction.user.id.into(),
			command,
		)
		.await?;
		respond_with_embed(
			http,
			app_interaction,
			ResponseOptions::CreateOrignial(true),
			|embed| {
				create_error_embed(
					embed,
					"Cool down! You need to wait a bit more before using this command.",
					format!("Cooldown has not been reached: {remaining_cooldown}s left."),
					Some("Just pipe down and wait!"),
				)
			},
		)
		.await
		.map(|_| ())
	}

	/// Handles a command execution for Aegistrate.
	async fn handle_command_execution(
		command: Box<dyn Command>,
		context: &Context,
		app_interaction: &ApplicationCommandInteraction,
	) {
		let command_id = app_interaction.id;
		let command_name = command.metadata().name;
		match command.execute(context, app_interaction).await {
			Ok(_) => {
				info!("Interaction {command_id}: Command /{command_name} executed successfully");
			},
			Err(why) => {
				error!(
					"Interaction {command_id}: Command /{command_name} executed with an error: \
					 {why}"
				);
			},
		}
		let _ = use_last(
			app_interaction.guild_id.unwrap().into(),
			app_interaction.user.id.into(),
			command.as_ref(),
		)
		.await;
	}

	/// Runs an application command for Aegistrate.
	async fn run_application_command(
		context: &Context,
		app_interaction: &ApplicationCommandInteraction,
	) {
		let http = context.http();
		if !READY_TO_GO.load(Ordering::Relaxed) {
			let _ = Self::not_ready_respond(http, app_interaction).await;
			return;
		}

		let command_name = &app_interaction.data.name;
		let Some(command) = command_by_name(command_name) else {
			let _ = Self::unknown_command_respond(http, app_interaction, command_name).await;
			return;
		};

		let executioner_id = app_interaction.user.id;
		let guild_id = app_interaction.guild_id.unwrap();
		if !cooled_down(guild_id.into(), executioner_id.into(), command.as_ref())
			.await
			.unwrap()
		{
			let _ = Self::not_cooled_down_respond(http, app_interaction, command.as_ref()).await;
			return;
		}

		Self::handle_command_execution(command, context, app_interaction).await;
	}
}

/// Sets up the database service portion for Aegistrate.
#[allow(clippy::missing_errors_doc)]
pub async fn set_up_database() -> Aegis<()> {
	let _ = MONGODB_CLIENT.set(mongod::Client::from_client(
		mongodb::Client::with_uri_str(var("MONGODB_URI")?).await?,
		"development",
	));
	Ok(())
}

/// Gets the MongoDB client that is under a layer of [`OnceCell`].
///
/// # Panics
///
/// This function panics if the [`MONGODB_CLIENT`] static variable
/// hasn't been initialized.
pub fn get_mongodb_client<'a>() -> &'a mongod::Client {
	MONGODB_CLIENT.get().unwrap()
}

/// Gets the Aegistrate user that is under a layer of [`OnceCell`].
///
/// # Panics
///
/// This function panics if the [`AEGISTRATE_USER`] static variable somehow
/// hasn't been initialized.
pub fn get_aegistrate_user<'a>() -> &'a CurrentUser {
	AEGISTRATE_USER.get().unwrap()
}

/// Controls whether Aegistrate is up and running.
pub static READY_TO_GO: AtomicBool = AtomicBool::new(false);

/// The interval to sleep for between each command registration.
pub static REGISTER_COMMAND_INTERVAL: f32 = 1.0;

/// Controls whether the Discord service portion is ready to go.
/// Take a look at [`spawn_timeout_checker`] to see how this variable is used.
pub static DISCORD_READY: AtomicBool = AtomicBool::new(false);

/// The bot user that Aegistrate assumes identity of.
/// To access with knowledge that it has been initialized, use
/// [`get_aegistrate_user`].
pub static AEGISTRATE_USER: OnceCell<CurrentUser> = OnceCell::const_new();

/// The MongoDB client that will interact with Aegistrate's MongoDB database.
pub static MONGODB_CLIENT: OnceCell<mongod::Client> = OnceCell::const_new();
